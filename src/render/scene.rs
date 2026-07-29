use std::path::Path;

use taffy::TaffyTree;
use vello::Scene as VelloScene;

use crate::render::{
    animation::FpsTarget,
    components::SceneGpuRenderer,
    error::RenderError,
    frame::{Component, Frame},
    models::Size,
};

pub struct RenderedFrame {
    pub scene: VelloScene,
    pub size: Size,
}

pub struct Scene {
    frames: Vec<RenderedFrame>,
    fps: FpsTarget,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            fps: FpsTarget::F30,
        }
    }

    pub fn set_fps(&mut self, fps: FpsTarget) {
        self.fps = fps;
    }

    pub fn push_frame(&mut self, mut frame: Frame) -> Result<(), RenderError> {
        let mut tree = TaffyTree::new();
        frame.content.register_layout(&mut tree)?;
        let scene = frame.content.render(&tree)?;
        let size = frame.content.size()?;
        self.frames.push(RenderedFrame { scene, size });
        Ok(())
    }

    pub fn append(&mut self, mut scene: Scene) -> Result<(), RenderError> {
        if self.fps != scene.fps {
            return Err(RenderError::FpsDismatch);
        }
        self.frames.append(&mut scene.frames);
        Ok(())
    }

    pub fn render(&self, path: &Path) -> Result<(), RenderError> {
        ffmpeg_next::init().map_err(|_| RenderError::FfmpegError)?;

        if self.frames.is_empty() {
            return Ok(());
        }

        // Determine even dimensions based on the first frame
        let width = (self.frames[0].size.width as u32) / 2 * 2;
        let height = (self.frames[0].size.height as u32) / 2 * 2;

        let mut octx = ffmpeg_next::format::output(path)
            .map_err(|e| RenderError::CustomError(format!("Cannot open output file: {}", e)))?;

        let codec = ffmpeg_next::codec::encoder::find(ffmpeg_next::codec::Id::H264)
            .ok_or_else(|| RenderError::CustomError("H264 encoder not found".to_string()))?;

        let encoder_ctx = ffmpeg_next::codec::context::Context::new_with_codec(codec);
        let mut encoder = encoder_ctx
            .encoder()
            .video()
            .map_err(|e| RenderError::CustomError(format!("Cannot create video encoder: {}", e)))?;

        encoder.set_width(width);
        encoder.set_height(height);
        encoder.set_format(ffmpeg_next::format::Pixel::YUV420P);

        let fps = self.fps.value();
        encoder.set_frame_rate(Some(ffmpeg_next::Rational::new(fps as i32, 1)));
        encoder.set_time_base(ffmpeg_next::Rational::new(1, fps as i32));

        if octx
            .format()
            .flags()
            .contains(ffmpeg_next::format::Flags::GLOBAL_HEADER)
        {
            encoder.set_flags(ffmpeg_next::codec::Flags::GLOBAL_HEADER);
        }

        let mut opened_encoder = encoder
            .open()
            .map_err(|e| RenderError::CustomError(format!("Cannot open encoder: {}", e)))?;

        let ost_index = {
            let mut ost = octx
                .add_stream(codec)
                .map_err(|e| RenderError::CustomError(format!("Cannot add video stream: {}", e)))?;
            ost.set_parameters(&opened_encoder);
            ost.index()
        };

        octx.write_header()
            .map_err(|e| RenderError::CustomError(format!("Cannot write header: {}", e)))?;

        let ost_time_base = octx.stream(ost_index).unwrap().time_base();

        let mut scaler = ffmpeg_next::software::scaling::Context::get(
            ffmpeg_next::format::Pixel::RGBA,
            width,
            height,
            ffmpeg_next::format::Pixel::YUV420P,
            width,
            height,
            ffmpeg_next::software::scaling::Flags::BILINEAR,
        )
        .map_err(|e| RenderError::CustomError(format!("Cannot create scaler: {}", e)))?;

        let mut gpu_renderer = SceneGpuRenderer::new()?;

        for (i, rendered_frame) in self.frames.iter().enumerate() {
            let image = gpu_renderer.render_to_image(&rendered_frame.scene, width, height)?;

            // Load RGBA into input frame
            let mut src_frame =
                ffmpeg_next::frame::Video::new(ffmpeg_next::format::Pixel::RGBA, width, height);

            let stride = src_frame.stride(0);
            let data = src_frame.data_mut(0);
            let raw_pixels = image.as_raw();

            for y in 0..height as usize {
                let src_start = y * (width as usize * 4);
                let src_end = src_start + (width as usize * 4);
                let dst_start = y * stride;
                let dst_end = dst_start + (width as usize * 4);
                data[dst_start..dst_end].copy_from_slice(&raw_pixels[src_start..src_end]);
            }

            let mut dst_frame =
                ffmpeg_next::frame::Video::new(ffmpeg_next::format::Pixel::YUV420P, width, height);

            scaler
                .run(&src_frame, &mut dst_frame)
                .map_err(|e| RenderError::CustomError(format!("Scaler failed: {}", e)))?;

            dst_frame.set_pts(Some(i as i64));

            opened_encoder.send_frame(&dst_frame).map_err(|e| {
                RenderError::CustomError(format!("Encoder send frame failed: {}", e))
            })?;

            let mut packet = ffmpeg_next::Packet::empty();
            while opened_encoder.receive_packet(&mut packet).is_ok() {
                packet.rescale_ts(opened_encoder.time_base(), ost_time_base);
                packet.set_stream(ost_index);
                packet
                    .write_interleaved(&mut octx)
                    .map_err(|e| RenderError::CustomError(format!("Cannot write packet: {}", e)))?;
            }
        }

        // Flush the encoder
        opened_encoder
            .send_eof()
            .map_err(|e| RenderError::CustomError(format!("Encoder flush failed: {}", e)))?;

        let mut packet = ffmpeg_next::Packet::empty();
        while opened_encoder.receive_packet(&mut packet).is_ok() {
            packet.rescale_ts(opened_encoder.time_base(), ost_time_base);
            packet.set_stream(ost_index);
            packet
                .write_interleaved(&mut octx)
                .map_err(|e| RenderError::CustomError(format!("Cannot write packet: {}", e)))?;
        }

        octx.write_trailer()
            .map_err(|e| RenderError::CustomError(format!("Cannot write trailer: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use crate::render::{
        components::{
            freebox::{Coordinate, LocatedComponent, Location},
            text::Text,
        },
        error::RenderError,
        frame::{Component, Frame},
        models::Size,
    };

    #[test]
    fn test_render_video_with_located_component_placeholder() -> Result<(), RenderError> {
        let text = Text::new("Moving Text").size(48).color("#122422");
        let located = LocatedComponent::new(text).location(Location::new(
            Coordinate::absolute(100.0),
            Coordinate::absolute(100.0),
        ));
        let placeholder = located.to_placeholder()?;
        let controller = placeholder.pointer();

        let mut scene = Scene::new();

        for i in 0..30 {
            let frame = Frame::new(Size::youtube_shorts()).add(placeholder.clone());
            scene.push_frame(frame)?;

            // Edit position of LocatedComponent using placeholder pointer
            controller.with_mut(|lc| {
                lc.set_location(Location::new(
                    Coordinate::absolute(100.0 + (i as f64 * 10.0)),
                    Coordinate::absolute(100.0 + (i as f64 * 15.0)),
                ));
            })?;
        }

        let output_dir = Path::new("./tests");
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).ok();
        }
        let output_path = output_dir.join("test_located_component.mp4");
        scene.render(&output_path)?;

        assert!(output_path.exists());
        Ok(())
    }
}
