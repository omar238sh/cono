use std::{
    ops::{Add, Mul},
    path::Path,
    sync::{Arc, mpsc::sync_channel},
    thread,
    time::Instant,
};
use taffy::TaffyTree;
use vello::Scene as VelloScene;

use crate::render::{
    animation::FpsTarget,
    components::SceneGpuRenderer,
    error::RenderError,
    frame::{Component, Frame},
    models::Size,
};

#[derive(Clone)]
pub struct RenderedFrame {
    pub scene: VelloScene,
    pub size: Size,
}

impl mlua::FromLua for RenderedFrame {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "RenderedFrame".to_string(),
            message: Some("expected RenderedFrame UserData".to_string()),
        })
    }
}

#[derive(Clone)]
pub struct Scene {
    pub(crate) frames: Vec<RenderedFrame>,
    pub(crate) fps: FpsTarget,
}

impl mlua::FromLua for Scene {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Scene".to_string(),
            message: Some("expected Scene UserData".to_string()),
        })
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            fps: FpsTarget::F30,
        }
    }

    pub fn fps(&self) -> FpsTarget {
        match &self.fps {
            FpsTarget::F30 => FpsTarget::F30,
            FpsTarget::F45 => FpsTarget::F45,
            FpsTarget::F60 => FpsTarget::F60,
            FpsTarget::F120 => FpsTarget::F120,
            FpsTarget::Custom(v) => FpsTarget::Custom(*v),
        }
    }

    pub fn set_fps(&mut self, fps: FpsTarget) {
        self.fps = fps;
    }

    pub fn frames(&self) -> &[RenderedFrame] {
        &self.frames
    }

    pub fn frames_count(&self) -> usize {
        self.frames.len()
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

    // pub fn render(&self, path: &Path) -> Result<(), RenderError> {
    //     ffmpeg_next::init().map_err(|_| RenderError::FfmpegError)?;

    //     if self.frames.is_empty() {
    //         return Ok(());
    //     }

    //     // Determine even dimensions based on the first frame
    //     let width = (self.frames[0].size.width as u32) / 2 * 2;
    //     let height = (self.frames[0].size.height as u32) / 2 * 2;

    //     let mut octx = ffmpeg_next::format::output(path)
    //         .map_err(|e| RenderError::CustomError(format!("Cannot open output file: {}", e)))?;

    //     let codec = ffmpeg_next::codec::encoder::find_by_name("h264_nvenc")
    //         .or_else(|| ffmpeg_next::codec::encoder::find_by_name("h264_amf"))
    //         .or_else(|| ffmpeg_next::codec::encoder::find_by_name("h264_qsv"))
    //         .or_else(|| ffmpeg_next::codec::encoder::find(ffmpeg_next::codec::Id::H264))
    //         .ok_or_else(|| {
    //             RenderError::CustomError("No suitable H264 encoder found".to_string())
    //         })?;

    //     let encoder_ctx = ffmpeg_next::codec::context::Context::new_with_codec(codec);
    //     let mut opts = ffmpeg_next::Dictionary::new();
    //     opts.set("preset", "p4");
    //     opts.set("tune", "hq");
    //     let mut encoder = encoder_ctx
    //         .encoder()
    //         .video()
    //         .map_err(|e| RenderError::CustomError(format!("Cannot create video encoder: {}", e)))?;

    //     encoder.set_width(width);
    //     encoder.set_height(height);
    //     encoder.set_format(ffmpeg_next::format::Pixel::YUV420P);

    //     let fps = self.fps.value();
    //     encoder.set_frame_rate(Some(ffmpeg_next::Rational::new(fps as i32, 1)));
    //     encoder.set_time_base(ffmpeg_next::Rational::new(1, fps as i32));

    //     if octx
    //         .format()
    //         .flags()
    //         .contains(ffmpeg_next::format::Flags::GLOBAL_HEADER)
    //     {
    //         encoder.set_flags(ffmpeg_next::codec::Flags::GLOBAL_HEADER);
    //     }

    //     let mut opened_encoder = encoder
    //         .open_with(opts)
    //         .map_err(|e| RenderError::CustomError(format!("Cannot open encoder: {}", e)))?;

    //     let ost_index = {
    //         let mut ost = octx
    //             .add_stream(codec)
    //             .map_err(|e| RenderError::CustomError(format!("Cannot add video stream: {}", e)))?;
    //         ost.set_parameters(&opened_encoder);
    //         ost.index()
    //     };

    //     octx.write_header()
    //         .map_err(|e| RenderError::CustomError(format!("Cannot write header: {}", e)))?;

    //     let ost_time_base = octx.stream(ost_index).unwrap().time_base();

    //     let mut scaler = ffmpeg_next::software::scaling::Context::get(
    //         ffmpeg_next::format::Pixel::RGBA,
    //         width,
    //         height,
    //         ffmpeg_next::format::Pixel::YUV420P,
    //         width,
    //         height,
    //         ffmpeg_next::software::scaling::Flags::BILINEAR,
    //     )
    //     .map_err(|e| RenderError::CustomError(format!("Cannot create scaler: {}", e)))?;

    //     let mut gpu_renderer = SceneGpuRenderer::new()?;
    //     let mut cache = Vec::new();
    //     for rendered_frame in self.frames.iter() {
    //         let image = gpu_renderer.render_to_image(&rendered_frame.scene, width, height)?;
    //         cache.push(image);
    //     }
    //     for (i, image) in cache.iter().enumerate() {
    //         let mut src_frame =
    //             ffmpeg_next::frame::Video::new(ffmpeg_next::format::Pixel::RGBA, width, height);

    //         let stride = src_frame.stride(0);
    //         let data = src_frame.data_mut(0);
    //         let raw_pixels = image.as_raw();

    //         for y in 0..height as usize {
    //             let src_start = y * (width as usize * 4);
    //             let src_end = src_start + (width as usize * 4);
    //             let dst_start = y * stride;
    //             let dst_end = dst_start + (width as usize * 4);
    //             data[dst_start..dst_end].copy_from_slice(&raw_pixels[src_start..src_end]);
    //         }

    //         let mut dst_frame =
    //             ffmpeg_next::frame::Video::new(ffmpeg_next::format::Pixel::YUV420P, width, height);

    //         scaler
    //             .run(&src_frame, &mut dst_frame)
    //             .map_err(|e| RenderError::CustomError(format!("Scaler failed: {}", e)))?;

    //         dst_frame.set_pts(Some(i as i64));

    //         opened_encoder.send_frame(&dst_frame).map_err(|e| {
    //             RenderError::CustomError(format!("Encoder send frame failed: {}", e))
    //         })?;

    //         let mut packet = ffmpeg_next::Packet::empty();
    //         while opened_encoder.receive_packet(&mut packet).is_ok() {
    //             packet.rescale_ts(opened_encoder.time_base(), ost_time_base);
    //             packet.set_stream(ost_index);
    //             packet
    //                 .write_interleaved(&mut octx)
    //                 .map_err(|e| RenderError::CustomError(format!("Cannot write packet: {}", e)))?;
    //         }
    //     }

    //     // Flush the encoder
    //     opened_encoder
    //         .send_eof()
    //         .map_err(|e| RenderError::CustomError(format!("Encoder flush failed: {}", e)))?;

    //     let mut packet = ffmpeg_next::Packet::empty();
    //     while opened_encoder.receive_packet(&mut packet).is_ok() {
    //         packet.rescale_ts(opened_encoder.time_base(), ost_time_base);
    //         packet.set_stream(ost_index);
    //         packet
    //             .write_interleaved(&mut octx)
    //             .map_err(|e| RenderError::CustomError(format!("Cannot write packet: {}", e)))?;
    //     }

    //     octx.write_trailer()
    //         .map_err(|e| RenderError::CustomError(format!("Cannot write trailer: {}", e)))?;

    //     Ok(())
    // }

    pub fn render(&self, path: &Path) -> Result<(), RenderError> {
        ffmpeg_next::init().map_err(|_| RenderError::FfmpegError)?;

        if self.frames.is_empty() {
            return Ok(());
        }

        let width = (self.frames[0].size.width as u32) / 2 * 2;
        let height = (self.frames[0].size.height as u32) / 2 * 2;
        let fps = self.fps.value();
        let path_buf = path.to_path_buf();

        let (tx, rx) = sync_channel::<Vec<u8>>(16);

        let encoder_handle = thread::spawn(move || -> Result<(), RenderError> {
            let mut octx = ffmpeg_next::format::output(&path_buf)
                .map_err(|e| RenderError::CustomError(format!("Cannot open output file: {}", e)))?;

            let codec = ffmpeg_next::codec::encoder::find_by_name("h264_nvenc")
                .or_else(|| ffmpeg_next::codec::encoder::find_by_name("h264_amf"))
                .or_else(|| ffmpeg_next::codec::encoder::find_by_name("h264_qsv"))
                .or_else(|| ffmpeg_next::codec::encoder::find(ffmpeg_next::codec::Id::H264))
                .ok_or_else(|| RenderError::CustomError("No encoder found".to_string()))?;

            let encoder_ctx = ffmpeg_next::codec::context::Context::new_with_codec(codec);
            let mut opts = ffmpeg_next::Dictionary::new();

            let target_pix_fmt = if codec.name().contains("nvenc") {
                opts.set("preset", "p1");
                opts.set("tune", "ull");
                opts.set("zerolatency", "1");
                ffmpeg_next::format::Pixel::NV12
            } else {
                opts.set("preset", "ultrafast");
                opts.set("tune", "zerolatency");
                opts.set("crf", "23");
                ffmpeg_next::format::Pixel::YUV420P
            };

            let mut encoder = encoder_ctx.encoder().video().map_err(|e| {
                RenderError::CustomError(format!("Cannot create video encoder: {}", e))
            })?;

            encoder.set_width(width);
            encoder.set_height(height);
            encoder.set_format(target_pix_fmt);
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
                .open_with(opts)
                .map_err(|e| RenderError::CustomError(format!("Cannot open encoder: {}", e)))?;

            let ost_index = {
                let mut ost = octx
                    .add_stream(codec)
                    .map_err(|e| RenderError::CustomError(format!("Cannot add stream: {}", e)))?;
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
                target_pix_fmt,
                width,
                height,
                ffmpeg_next::software::scaling::Flags::POINT,
            )
            .map_err(|e| RenderError::CustomError(format!("Cannot create scaler: {}", e)))?;

            let mut src_frame =
                ffmpeg_next::frame::Video::new(ffmpeg_next::format::Pixel::RGBA, width, height);
            let mut dst_frame = ffmpeg_next::frame::Video::new(target_pix_fmt, width, height);
            let mut packet = ffmpeg_next::Packet::empty();

            let stride = src_frame.stride(0);
            let row_len = width as usize * 4;
            let mut frame_idx = 0i64;

            while let Ok(raw_pixels) = rx.recv() {
                let data = src_frame.data_mut(0);
                if stride == row_len {
                    data[..raw_pixels.len()].copy_from_slice(&raw_pixels);
                } else {
                    for (dst_row, src_row) in data
                        .chunks_exact_mut(stride)
                        .zip(raw_pixels.chunks_exact(row_len))
                    {
                        dst_row[..row_len].copy_from_slice(src_row);
                    }
                }

                scaler
                    .run(&src_frame, &mut dst_frame)
                    .map_err(|e| RenderError::CustomError(format!("Scaler failed: {}", e)))?;

                dst_frame.set_pts(Some(frame_idx));
                frame_idx += 1;

                opened_encoder.send_frame(&dst_frame).map_err(|e| {
                    RenderError::CustomError(format!("Encoder send frame failed: {}", e))
                })?;

                while opened_encoder.receive_packet(&mut packet).is_ok() {
                    packet.rescale_ts(opened_encoder.time_base(), ost_time_base);
                    packet.set_stream(ost_index);
                    packet.write_interleaved(&mut octx).map_err(|e| {
                        RenderError::CustomError(format!("Cannot write packet: {}", e))
                    })?;
                }
            }

            opened_encoder
                .send_eof()
                .map_err(|e| RenderError::CustomError(format!("Encoder flush failed: {}", e)))?;

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
        });

        let mut gpu_renderer = SceneGpuRenderer::new()?;
        let scenes = self.frames.iter().map(|f| Arc::new(f.scene.clone()));

        gpu_renderer.render_all_to_channel(scenes, width, height, tx)?;
        // drop(tx);
        encoder_handle.join().unwrap()?;

        Ok(())
    }

    pub fn render_video(&self, path: &Path) -> Result<(), RenderError> {
        self.render(path)
    }
}

pub fn add_scenes(a: &Scene, b: &Scene) -> Scene {
    let mut combined = a.clone();
    if combined.frames.is_empty() {
        return b.clone();
    }
    if b.frames.is_empty() {
        return combined;
    }
    let _ = combined.append(b.clone());
    combined
}

impl Add<Scene> for Scene {
    type Output = Scene;
    fn add(self, rhs: Scene) -> Self::Output {
        add_scenes(&self, &rhs)
    }
}

impl Add<&Scene> for Scene {
    type Output = Scene;
    fn add(self, rhs: &Scene) -> Self::Output {
        add_scenes(&self, rhs)
    }
}

impl Add<Scene> for &Scene {
    type Output = Scene;
    fn add(self, rhs: Scene) -> Self::Output {
        add_scenes(self, &rhs)
    }
}

impl Add<&Scene> for &Scene {
    type Output = Scene;
    fn add(self, rhs: &Scene) -> Self::Output {
        add_scenes(self, rhs)
    }
}

pub fn multiply_scenes(a: &Scene, b: &Scene) -> Scene {
    let len_a = a.frames.len();
    let len_b = b.frames.len();

    if len_a == 0 && len_b == 0 {
        return Scene::new();
    }
    if len_a == 0 {
        return b.clone();
    }
    if len_b == 0 {
        return a.clone();
    }

    let max_len = std::cmp::max(len_a, len_b);
    let mut combined_frames = Vec::with_capacity(max_len);

    for i in 0..max_len {
        let frame_a = &a.frames[if len_a == 1 { 0 } else { i.min(len_a - 1) }];
        let frame_b = &b.frames[if len_b == 1 { 0 } else { i.min(len_b - 1) }];

        let mut vello_scene = frame_a.scene.clone();
        vello_scene.append(&frame_b.scene, None);

        combined_frames.push(RenderedFrame {
            scene: vello_scene,
            size: frame_a.size,
        });
    }

    Scene {
        frames: combined_frames,
        fps: a.fps(),
    }
}

impl Mul<Scene> for Scene {
    type Output = Scene;
    fn mul(self, rhs: Scene) -> Self::Output {
        multiply_scenes(&self, &rhs)
    }
}

impl Mul<&Scene> for Scene {
    type Output = Scene;
    fn mul(self, rhs: &Scene) -> Self::Output {
        multiply_scenes(&self, rhs)
    }
}

impl Mul<Scene> for &Scene {
    type Output = Scene;
    fn mul(self, rhs: Scene) -> Self::Output {
        multiply_scenes(self, &rhs)
    }
}

impl Mul<&Scene> for &Scene {
    type Output = Scene;
    fn mul(self, rhs: &Scene) -> Self::Output {
        multiply_scenes(self, rhs)
    }
}

impl mlua::UserData for RenderedFrame {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.size));
        fields.add_field_method_set("size", |_, this, size: Size| {
            this.size = size;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("size", |_, this, ()| Ok(this.size));
    }
}

impl mlua::UserData for Scene {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("fps", |_, this| Ok(this.fps().value()));
        fields.add_field_method_set("fps", |_, this, fps: u32| {
            this.set_fps(FpsTarget::Custom(fps));
            Ok(())
        });
        fields.add_field_method_get("frames_count", |_, this| Ok(this.frames_count()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, ()| Ok(Scene::new()));

        methods.add_method("fps", |_lua, this, ()| Ok(this.fps().value()));

        methods.add_method_mut("set_fps", |_lua, this, fps: u32| {
            this.set_fps(FpsTarget::Custom(fps));
            Ok(())
        });

        methods.add_method("frames_count", |_lua, this, ()| Ok(this.frames_count()));

        methods.add_method_mut("push_frame", |_lua, this, frame: Frame| {
            this.push_frame(frame)
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_method_mut("append", |_lua, this, other: Scene| {
            this.append(other)
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_method("render", |_lua, this, path_str: String| {
            this.render(Path::new(&path_str))
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_method("render_video", |_lua, this, path_str: String| {
            this.render_video(Path::new(&path_str))
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_meta_method(mlua::MetaMethod::Mul, |_, this, other: Scene| {
            Ok(this.clone() * other)
        });

        methods.add_meta_method(mlua::MetaMethod::Add, |_, this, other_val: mlua::Value| {
            if let mlua::Value::UserData(ud) = &other_val {
                if let Ok(other_scene) = ud.borrow::<Scene>() {
                    return Ok(this.clone() + other_scene.clone());
                }
                if let Ok(other_model) = ud.borrow::<crate::render::animation::Model>() {
                    return Ok(this.clone() + other_model.clone());
                }
            }
            Err(mlua::Error::external(
                "Invalid operand for Scene addition (+)",
            ))
        });
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
        let text = Text::new("Moving Text").size(48).color("#ffffff");
        let located = LocatedComponent::new(text).location(Location::new(
            Coordinate::absolute(100.0),
            Coordinate::absolute(100.0),
        ));
        let placeholder = located.to_placeholder()?;
        let controller = placeholder.pointer();

        let mut scene = Scene::new();
        scene.set_fps(FpsTarget::Custom(120));

        for i in 0..120 {
            let frame = Frame::new(Size::youtube_shorts()).add(placeholder.clone());
            scene.push_frame(frame)?;

            // Edit position of LocatedComponent using placeholder pointer
            controller.with_mut(|lc| {
                lc.shift(Location::new(
                    Coordinate::absolute(i as f64 * 5.0),
                    Coordinate::absolute(i as f64 * 5.0),
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
