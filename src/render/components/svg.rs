use image::RgbaImage;
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene, util::RenderContext, wgpu};
use vello_svg as vsvg;

use crate::render::{error::RenderError, frame::Component, models::Size};

pub struct SvgBox {
    scene: Scene,
    size: Size,
}

impl Component for SvgBox {
    fn render(&self) -> Scene {
        self.scene.clone()
    }

    fn size(&self) -> Size {
        self.size.clone()
    }
}

impl SvgBox {
    pub fn new(content: String) -> Result<Self, RenderError> {
        let tree = vsvg::usvg::Tree::from_str(&content, &vsvg::usvg::Options::default())?;
        let width = tree.size().width().ceil() as u32;
        let height = tree.size().height().ceil() as u32;

        let scene = vsvg::render_tree(&tree);

        Ok(Self {
            scene,
            size: Size::new(width, height),
        })
    }
}

pub struct SceneGpuRenderer {
    context: RenderContext,
    renderer: Renderer,
    dev_id: usize,
}

impl SceneGpuRenderer {
    pub fn new() -> Result<Self, RenderError> {
        let mut context = RenderContext::new();
        let dev_id = pollster::block_on(context.device(None)).ok_or_else(|| {
            RenderError::CustomError("Cannot create compatible GPU device".to_string())
        })?;
        let device_handle = &context.devices[dev_id];

        let renderer = Renderer::new(&device_handle.device, RendererOptions::default())
            .map_err(|e| RenderError::CustomError(format!("Cannot create Vello renderer: {e}")))?;

        Ok(Self {
            context,
            renderer,
            dev_id,
        })
    }

    pub fn render_to_image(
        &mut self,
        scene: &Scene,
        width: u32,
        height: u32,
    ) -> Result<RgbaImage, RenderError> {
        if width == 0 || height == 0 {
            return Ok(RgbaImage::new(width, height));
        }

        let device_handle = &self.context.devices[self.dev_id];

        let texture = device_handle
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("svg-target-texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.renderer
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                scene,
                &texture_view,
                &RenderParams {
                    base_color: vello::peniko::Color::TRANSPARENT,
                    width,
                    height,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .map_err(|e| RenderError::CustomError(format!("Cannot render SVG scene: {e}")))?;

        let bytes_per_pixel = 4u32;
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
        let buffer_size = (padded_bytes_per_row * height) as u64;

        let output_buffer = device_handle.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("svg-readback-buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder =
            device_handle
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("svg-readback-encoder"),
                });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        device_handle.queue.submit(Some(encoder.finish()));

        let slice = output_buffer.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device_handle
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| RenderError::CustomError(format!("Cannot wait for GPU readback: {e}")))?;

        let mapped = slice.get_mapped_range();

        let mut data = vec![0; (width * height * 4) as usize];
        for row in 0..height as usize {
            let src_offset = row * padded_bytes_per_row as usize;
            let dst_offset = row * unpadded_bytes_per_row as usize;
            data[dst_offset..dst_offset + unpadded_bytes_per_row as usize]
                .copy_from_slice(&mapped[src_offset..src_offset + unpadded_bytes_per_row as usize]);
        }
        drop(mapped);
        output_buffer.unmap();

        let image = RgbaImage::from_raw(width, height, data).ok_or_else(|| {
            RenderError::CustomError("Cannot create RgbaImage from raw data".to_string())
        })?;

        Ok(image)
    }
}

#[cfg(test)]
mod cvg_test {
    use chrono::DateTime;
    use vello::peniko::color::palette::css::SNOW;

    use crate::render::{
        components::svg::{SceneGpuRenderer, SvgBox},
        error::RenderError,
        frame::Component,
        typst::{LuaWorld, pager::Pager},
    };

    #[test]
    fn test_svg() -> Result<(), RenderError> {
        let content = LuaWorld::new(
            Pager::new()
                .add("مرحبا")
                .add("Hello")
                .add("eee")
                .add("OOO")
                .add("سلام عليكم"),
        )
        .to_svgs()
        .map_err(|_| RenderError::CustomError("cannot create svgs".to_string()))?;
        let mut renderer = SceneGpuRenderer::new()?;
        for svg in content {
            let mysvg_box = SvgBox::new(svg)?;
            let p = mysvg_box.render();
            let size = mysvg_box.size();
            let image = renderer.render_to_image(&p, size.width, size.height)?;
            image
                .save("./tests/svg.png")
                .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;
        }
        Ok(())
    }
}
