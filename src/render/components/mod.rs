use image::RgbaImage;
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene, util::RenderContext, wgpu};

use crate::render::error::RenderError;

pub mod column;
pub mod container;
pub mod freebox;
pub mod row;
pub mod style;
pub mod svg;
pub mod text;

struct RenderTargetCache {
    width: u32,
    height: u32,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    buffer: wgpu::Buffer,
    padded_bytes_per_row: u32,
    unpadded_bytes_per_row: u32,
}

pub struct SceneGpuRenderer {
    context: RenderContext,
    renderer: Renderer,
    dev_id: usize,

    cache: Option<RenderTargetCache>,
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
            cache: None,
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
        let device = &device_handle.device;
        let queue = &device_handle.queue;

        let needs_new_cache = self
            .cache
            .as_ref()
            .map_or(true, |c| c.width != width || c.height != height);

        if needs_new_cache {
            let bytes_per_pixel = 4u32;
            let unpadded_bytes_per_row = width * bytes_per_pixel;
            let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
            let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
            let buffer_size = (padded_bytes_per_row * height) as u64;

            let texture = device.create_texture(&wgpu::TextureDescriptor {
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

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("svg-readback-buffer"),
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            self.cache = Some(RenderTargetCache {
                width,
                height,
                texture,
                view,
                buffer,
                padded_bytes_per_row,
                unpadded_bytes_per_row,
            });
        }

        let cache = self.cache.as_ref().unwrap();

        self.renderer
            .render_to_texture(
                device,
                queue,
                scene,
                &cache.view,
                &RenderParams {
                    base_color: vello::peniko::Color::TRANSPARENT,
                    width,
                    height,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .map_err(|e| RenderError::CustomError(format!("Cannot render SVG scene: {e}")))?;

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("svg-readback-encoder"),
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &cache.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &cache.buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(cache.padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        queue.submit(Some(encoder.finish()));

        let slice = cache.buffer.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});

        device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| RenderError::CustomError(format!("Cannot wait for GPU readback: {e}")))?;

        let mapped = slice.get_mapped_range();
        let mut data = vec![0; (width * height * 4) as usize];

        for row in 0..height as usize {
            let src_offset = row * cache.padded_bytes_per_row as usize;
            let dst_offset = row * cache.unpadded_bytes_per_row as usize;
            data[dst_offset..dst_offset + cache.unpadded_bytes_per_row as usize].copy_from_slice(
                &mapped[src_offset..src_offset + cache.unpadded_bytes_per_row as usize],
            );
        }

        drop(mapped);
        cache.buffer.unmap();

        let image = RgbaImage::from_raw(width, height, data).ok_or_else(|| {
            RenderError::CustomError("Cannot create RgbaImage from raw data".to_string())
        })?;

        Ok(image)
    }
}
