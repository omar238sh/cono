use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene, util::RenderContext};
use vello_svg as vsvg;

use crate::render::{error::RenderError, frame::Component, frame::Pixmap};

struct SvgBox {
    content: Pixmap,
}

impl Component for SvgBox {
    fn render(&self) -> &Pixmap {
        &self.content
    }

    fn content_mut(&mut self) -> &mut Pixmap {
        &mut self.content
    }
}

impl SvgBox {
    pub fn new(content: String) -> Result<Self, RenderError> {
        let tree = vsvg::usvg::Tree::from_str(&content, &vsvg::usvg::Options::default())?;
        let width = tree.size().width().ceil() as u32;
        let height = tree.size().height().ceil() as u32;
        let mut pixmap = Pixmap::new(width, height).ok_or(RenderError::CannotCreatePixmap)?;
        let scene = vsvg::render_tree(&tree);
        render_scene_to_pixmap(&scene, &mut pixmap)?;

        Ok(Self { content: pixmap })
    }
}

fn render_scene_to_pixmap(scene: &Scene, pixmap: &mut Pixmap) -> Result<(), RenderError> {
    let width = pixmap.width();
    let height = pixmap.height();
    if width == 0 || height == 0 {
        return Ok(());
    }

    let mut context = RenderContext::new();
    let dev_id = pollster::block_on(context.device(None)).ok_or_else(|| {
        RenderError::CustomError("Cannot create compatible GPU device".to_string())
    })?;
    let device_handle = &context.devices[dev_id];

    let mut renderer = Renderer::new(&device_handle.device, RendererOptions::default())
        .map_err(|e| RenderError::CustomError(format!("Cannot create Vello renderer: {e}")))?;

    let texture = device_handle
        .device
        .create_texture(&vello::wgpu::TextureDescriptor {
            label: Some("svg-target-texture"),
            size: vello::wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: vello::wgpu::TextureDimension::D2,
            format: vello::wgpu::TextureFormat::Rgba8Unorm,
            usage: vello::wgpu::TextureUsages::STORAGE_BINDING
                | vello::wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    let texture_view = texture.create_view(&vello::wgpu::TextureViewDescriptor::default());

    renderer
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
    let align = vello::wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
    let buffer_size = (padded_bytes_per_row * height) as u64;

    let output_buffer = device_handle
        .device
        .create_buffer(&vello::wgpu::BufferDescriptor {
            label: Some("svg-readback-buffer"),
            size: buffer_size,
            usage: vello::wgpu::BufferUsages::COPY_DST | vello::wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

    let mut encoder =
        device_handle
            .device
            .create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
                label: Some("svg-readback-encoder"),
            });
    encoder.copy_texture_to_buffer(
        vello::wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: vello::wgpu::Origin3d::ZERO,
            aspect: vello::wgpu::TextureAspect::All,
        },
        vello::wgpu::TexelCopyBufferInfo {
            buffer: &output_buffer,
            layout: vello::wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        vello::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    device_handle.queue.submit(Some(encoder.finish()));

    let slice = output_buffer.slice(..);
    slice.map_async(vello::wgpu::MapMode::Read, |_| {});
    device_handle
        .device
        .poll(vello::wgpu::PollType::wait_indefinitely())
        .map_err(|e| RenderError::CustomError(format!("Cannot wait for GPU readback: {e}")))?;

    let mapped = slice.get_mapped_range();
    let target = pixmap.data_mut();
    for row in 0..height as usize {
        let src_offset = row * padded_bytes_per_row as usize;
        let dst_offset = row * unpadded_bytes_per_row as usize;
        target[dst_offset..dst_offset + unpadded_bytes_per_row as usize]
            .copy_from_slice(&mapped[src_offset..src_offset + unpadded_bytes_per_row as usize]);
    }
    drop(mapped);
    output_buffer.unmap();

    Ok(())
}

#[cfg(test)]
mod cvg_test {
    use crate::render::{
        components::svg::SvgBox,
        error::RenderError,
        frame::Component,
        typst::{LuaWorld, pager::Pager},
    };

    #[test]
    fn test_svg() -> Result<(), RenderError> {
        let content = LuaWorld::new(Pager::new().add("مرحبا"))
            .to_svgs()
            .map_err(|_| RenderError::CustomError("cannot create svgs".to_string()))?;
        for svg in content {
            let mysvg_box = SvgBox::new(svg)?;
            let p = mysvg_box.render();
            p.save_png("./tests/svg.png")
                .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;
        }
        Ok(())
    }
}
