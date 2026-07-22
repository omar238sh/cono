use std::path::Path;

use crate::render::{animation::FpsTarget, error::RenderError, frame::Frame};

pub struct Scene {
    frames: Vec<Frame>,
    fps: FpsTarget,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            fps: FpsTarget::F30,
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
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
        let mut first_frame = Frame {
            content: self.frames[0].content.clone(),
        };
        let first_image = first_frame.to_image()?;
        let width = first_image.width() / 2 * 2;
        let height = first_image.height() / 2 * 2;

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

        let (ost_index, ost_time_base) = {
            let mut ost = octx
                .add_stream(codec)
                .map_err(|e| RenderError::CustomError(format!("Cannot add video stream: {}", e)))?;
            ost.set_parameters(&opened_encoder);
            (ost.index(), ost.time_base())
        };

        octx.write_header()
            .map_err(|e| RenderError::CustomError(format!("Cannot write header: {}", e)))?;

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

        for (i, frame) in self.frames.iter().enumerate() {
            let mut frame_clone = Frame {
                content: frame.content.clone(),
            };
            let image = frame_clone.to_image()?;

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
