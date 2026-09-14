use image::RgbaImage;
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene, util::RenderContext, wgpu};

use crate::render::error::RenderError;

// pub mod canvas;
pub mod column;
pub mod container;
pub mod freebox;
pub mod grid;
pub mod placeholder;
pub mod row;
pub mod style;
pub mod svg;
pub mod text;

#[macro_export]
macro_rules! down_cast {
    ($obj:expr, $method:ident, $val:expr) => {
        $crate::down_cast!(
            $obj, $method, $val => [
                Text,
                Grid,
                Column,
                Row,
                Container,
                FreeBox,
                SvgBox,
                LocatedComponent,
                PlaceHolder<LocatedComponent>,
                PlaceHolder<Text>,
                PlaceHolder<Grid>,
                PlaceHolder<Column>,
                PlaceHolder<Row>,
                PlaceHolder<Container>,
                PlaceHolder<FreeBox>,
                PlaceHolder<SvgBox>,
                ElementPointer<Text>,
                ElementPointer<Grid>,
                ElementPointer<Column>,
                ElementPointer<Row>,
                ElementPointer<Container>,
                ElementPointer<FreeBox>,
                ElementPointer<SvgBox>,
                ElementPointer<LocatedComponent>,
            ]
        );
    };

    ($val:expr, |$c:ident| $action:expr) => {
        $crate::down_cast!(
            $val, |$c| $action => [
                Text, Grid, Column, Row, Container, FreeBox, SvgBox, LocatedComponent,
                PlaceHolder<LocatedComponent>, PlaceHolder<Text>, PlaceHolder<Grid>, PlaceHolder<Column>,
                PlaceHolder<Row>, PlaceHolder<Container>, PlaceHolder<FreeBox>, PlaceHolder<SvgBox>
            ]
        )
    };

    ($obj:expr, $method:ident, $val:expr => [ $( $ty:ty ),* $(,)? ]) => {
        {
            use crate::render::components::{
                column::Column, container::Container, freebox::{FreeBox, LocatedComponent},
                grid::Grid, placeholder::PlaceHolder, row::Row, svg::SvgBox, text::Text
            };

            let mut handled = false;
            $(
                if !handled {
                    if let Ok(c) = $val.borrow::<$ty>() {
                        ($obj).$method((*c).clone());
                        handled = true;
                    }
                }
            )*
            if !handled {
                return Err(mlua::Error::UserDataTypeMismatch);
            }
        }
    };

    ($val:expr, |$c:ident| $action:expr => [ $( $ty:ty ),* $(,)? ]) => {
        {
            use crate::render::components::{
                column::Column, container::Container, freebox::{FreeBox, LocatedComponent}, grid::Grid, placeholder::PlaceHolder, row::Row, svg::SvgBox, text::Text
            };

            let mut result = None;
            $(
                if result.is_none() {
                    if let Ok($c) = $val.borrow::<$ty>() {
                        let $c = (*$c).clone();
                        result = Some($action);
                    }
                }
            )*
            result
        }
    };
}

// struct RenderTargetCache {
//     width: u32,
//     height: u32,
//     texture: wgpu::Texture,
//     view: wgpu::TextureView,
//     buffer: wgpu::Buffer,
//     padded_bytes_per_row: u32,
//     unpadded_bytes_per_row: u32,
// }

pub struct RenderTargetCache {
    pub width: u32,
    pub height: u32,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub buffers: [wgpu::Buffer; 2],
    pub padded_bytes_per_row: u32,
    pub unpadded_bytes_per_row: u32,
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
        let info = device_handle.adapter().get_info();
        println!(
            "Using GPU: {} ({:?}) - Backend: {:?}",
            info.name, info.device_type, info.backend
        );
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

            // let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            //     label: Some("svg-readback-buffer"),
            //     size: buffer_size,
            //     usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            //     mapped_at_creation: false,
            // });

            let buffer_0 = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("svg-readback-buffer-0"),
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            let buffer_1 = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("svg-readback-buffer-1"),
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            self.cache = Some(RenderTargetCache {
                width,
                height,
                texture,
                view,
                buffers: [buffer_0, buffer_1],
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
                buffer: &cache.buffers[0],
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

        let slice = cache.buffers[0].slice(..);
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
        cache.buffers[0].unmap();

        let image = RgbaImage::from_raw(width, height, data).ok_or_else(|| {
            RenderError::CustomError("Cannot create RgbaImage from raw data".to_string())
        })?;

        Ok(image)
    }

    pub fn render_all_to_channel(
        &mut self,
        scenes: impl IntoIterator<Item = std::sync::Arc<vello::Scene>>,
        width: u32,
        height: u32,
        tx: std::sync::mpsc::SyncSender<Vec<u8>>,
    ) -> Result<(), RenderError> {
        if width == 0 || height == 0 {
            return Ok(());
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

            let buffer_0 = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("svg-readback-buffer-0"),
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            let buffer_1 = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("svg-readback-buffer-1"),
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            self.cache = Some(RenderTargetCache {
                width,
                height,
                texture,
                view,
                buffers: [buffer_0, buffer_1],
                padded_bytes_per_row,
                unpadded_bytes_per_row,
            });
        }

        let cache = self.cache.as_ref().unwrap();
        let unpadded_len = cache.unpadded_bytes_per_row as usize;
        let padded_len = cache.padded_bytes_per_row as usize;
        let total_bytes = unpadded_len * height as usize;

        let mut total_rendered = 0;

        for (i, scene) in scenes.into_iter().enumerate() {
            total_rendered += 1;
            let write_buf_idx = i % 2;
            let read_buf_idx = (i + 1) % 2;

            self.renderer
                .render_to_texture(
                    device,
                    queue,
                    &scene,
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
                    buffer: &cache.buffers[write_buf_idx],
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

            let write_slice = cache.buffers[write_buf_idx].slice(..);
            write_slice.map_async(wgpu::MapMode::Read, |_| {});

            if i > 0 {
                device
                    .poll(wgpu::PollType::wait_indefinitely())
                    .map_err(|e| {
                        RenderError::CustomError(format!("Cannot wait for GPU readback: {e}"))
                    })?;

                let prev_slice = cache.buffers[read_buf_idx].slice(..);
                let mapped = prev_slice.get_mapped_range();
                let mut data = vec![0u8; total_bytes];

                if unpadded_len == padded_len {
                    data.copy_from_slice(&mapped[..total_bytes]);
                } else {
                    for row in 0..height as usize {
                        let src_offset = row * padded_len;
                        let dst_offset = row * unpadded_len;
                        data[dst_offset..dst_offset + unpadded_len]
                            .copy_from_slice(&mapped[src_offset..src_offset + unpadded_len]);
                    }
                }

                drop(mapped);
                cache.buffers[read_buf_idx].unmap();

                if tx.send(data).is_err() {
                    return Ok(());
                }
            }
        }

        if total_rendered > 0 {
            let last_idx = (total_rendered - 1) % 2;
            device
                .poll(wgpu::PollType::wait_indefinitely())
                .map_err(|e| {
                    RenderError::CustomError(format!("Cannot wait for GPU readback: {e}"))
                })?;

            let slice = cache.buffers[last_idx].slice(..);
            let mapped = slice.get_mapped_range();
            let mut data = vec![0u8; total_bytes];

            if unpadded_len == padded_len {
                data.copy_from_slice(&mapped[..total_bytes]);
            } else {
                for row in 0..height as usize {
                    let src_offset = row * padded_len;
                    let dst_offset = row * unpadded_len;
                    data[dst_offset..dst_offset + unpadded_len]
                        .copy_from_slice(&mapped[src_offset..src_offset + unpadded_len]);
                }
            }

            drop(mapped);
            cache.buffers[last_idx].unmap();

            let _ = tx.send(data);
        }

        Ok(())
    }

    // pub fn render_to_image(
    //     &mut self,
    //     scene: &Scene,
    //     width: u32,
    //     height: u32,
    // ) -> Result<RgbaImage, RenderError> {
    //     if width == 0 || height == 0 {
    //         return Ok(RgbaImage::new(width, height));
    //     }

    //     let device_handle = &self.context.devices[self.dev_id];
    //     let device = &device_handle.device;
    //     let queue = &device_handle.queue;

    //     let needs_new_cache = self
    //         .cache
    //         .as_ref()
    //         .map_or(true, |c| c.width != width || c.height != height);

    //     if needs_new_cache {
    //         let bytes_per_pixel = 4u32;
    //         let unpadded_bytes_per_row = width * bytes_per_pixel;
    //         let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    //         let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
    //         let buffer_size = (padded_bytes_per_row * height) as u64;

    //         let texture = device.create_texture(&wgpu::TextureDescriptor {
    //             label: Some("svg-target-texture"),
    //             size: wgpu::Extent3d {
    //                 width,
    //                 height,
    //                 depth_or_array_layers: 1,
    //             },
    //             mip_level_count: 1,
    //             sample_count: 1,
    //             dimension: wgpu::TextureDimension::D2,
    //             format: wgpu::TextureFormat::Rgba8Unorm,
    //             usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
    //             view_formats: &[],
    //         });

    //         let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    //         let buffer_0 = device.create_buffer(&wgpu::BufferDescriptor {
    //             label: Some("svg-readback-buffer-0"),
    //             size: buffer_size,
    //             usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
    //             mapped_at_creation: false,
    //         });
    //         let buffer_1 = device.create_buffer(&wgpu::BufferDescriptor {
    //             label: Some("svg-readback-buffer-1"),
    //             size: buffer_size,
    //             usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
    //             mapped_at_creation: false,
    //         });

    //         self.cache = Some(RenderTargetCache {
    //             width,
    //             height,
    //             texture,
    //             view,
    //             buffers: [buffer_0, buffer_1],
    //             padded_bytes_per_row,
    //             unpadded_bytes_per_row,
    //             current_buf: 0,
    //         });
    //     }

    //     let cache = self.cache.as_ref().unwrap();

    //     self.renderer
    //         .render_to_texture(
    //             device,
    //             queue,
    //             scene,
    //             &cache.view,
    //             &RenderParams {
    //                 base_color: vello::peniko::Color::TRANSPARENT,
    //                 width,
    //                 height,
    //                 antialiasing_method: AaConfig::Area,
    //             },
    //         )
    //         .map_err(|e| RenderError::CustomError(format!("Cannot render SVG scene: {e}")))?;

    //     let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //         label: Some("svg-readback-encoder"),
    //     });

    //     encoder.copy_texture_to_buffer(
    //         wgpu::TexelCopyTextureInfo {
    //             texture: &cache.texture,
    //             mip_level: 0,
    //             origin: wgpu::Origin3d::ZERO,
    //             aspect: wgpu::TextureAspect::All,
    //         },
    //         wgpu::TexelCopyBufferInfo {
    //             buffer: &cache.buffer,
    //             layout: wgpu::TexelCopyBufferLayout {
    //                 offset: 0,
    //                 bytes_per_row: Some(cache.padded_bytes_per_row),
    //                 rows_per_image: Some(height),
    //             },
    //         },
    //         wgpu::Extent3d {
    //             width,
    //             height,
    //             depth_or_array_layers: 1,
    //         },
    //     );
    //     queue.submit(Some(encoder.finish()));

    //     let slice = cache.buffer.slice(..);
    //     slice.map_async(wgpu::MapMode::Read, |_| {});

    //     device
    //         .poll(wgpu::PollType::wait_indefinitely())
    //         .map_err(|e| RenderError::CustomError(format!("Cannot wait for GPU readback: {e}")))?;

    //     let mapped = slice.get_mapped_range();
    //     let mut data = vec![0; (width * height * 4) as usize];

    //     for row in 0..height as usize {
    //         let src_offset = row * cache.padded_bytes_per_row as usize;
    //         let dst_offset = row * cache.unpadded_bytes_per_row as usize;
    //         data[dst_offset..dst_offset + cache.unpadded_bytes_per_row as usize].copy_from_slice(
    //             &mapped[src_offset..src_offset + cache.unpadded_bytes_per_row as usize],
    //         );
    //     }

    //     drop(mapped);
    //     cache.buffer.unmap();

    //     let image = RgbaImage::from_raw(width, height, data).ok_or_else(|| {
    //         RenderError::CustomError("Cannot create RgbaImage from raw data".to_string())
    //     })?;

    //     Ok(image)
    // }

    // pub fn render_to_raw_pixels(
    //     &mut self,
    //     scene: &Scene,
    //     width: u32,
    //     height: u32,
    //     output_buf: &mut [u8],
    // ) -> Result<(), RenderError> {
    //     if width == 0 || height == 0 {
    //         return Ok(());
    //     }

    //     let device_handle = &self.context.devices[self.dev_id];
    //     let device = &device_handle.device;
    //     let queue = &device_handle.queue;

    //     let needs_new_cache = self
    //         .cache
    //         .as_ref()
    //         .map_or(true, |c| c.width != width || c.height != height);

    //     if needs_new_cache {
    //         let bytes_per_pixel = 4u32;
    //         let unpadded_bytes_per_row = width * bytes_per_pixel;
    //         let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    //         let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
    //         let buffer_size = (padded_bytes_per_row * height) as u64;

    //         let texture = device.create_texture(&wgpu::TextureDescriptor {
    //             label: Some("svg-target-texture"),
    //             size: wgpu::Extent3d {
    //                 width,
    //                 height,
    //                 depth_or_array_layers: 1,
    //             },
    //             mip_level_count: 1,
    //             sample_count: 1,
    //             dimension: wgpu::TextureDimension::D2,
    //             format: wgpu::TextureFormat::Rgba8Unorm,
    //             usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
    //             view_formats: &[],
    //         });

    //         let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    //         let buffer = device.create_buffer(&wgpu::BufferDescriptor {
    //             label: Some("svg-readback-buffer"),
    //             size: buffer_size,
    //             usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
    //             mapped_at_creation: false,
    //         });

    //         self.cache = Some(RenderTargetCache {
    //             width,
    //             height,
    //             texture,
    //             view,
    //             buffer,
    //             padded_bytes_per_row,
    //             unpadded_bytes_per_row,
    //         });
    //     }

    //     let cache = self.cache.as_ref().unwrap();

    //     self.renderer
    //         .render_to_texture(
    //             device,
    //             queue,
    //             scene,
    //             &cache.view,
    //             &RenderParams {
    //                 base_color: vello::peniko::Color::TRANSPARENT,
    //                 width,
    //                 height,
    //                 antialiasing_method: AaConfig::Area,
    //             },
    //         )
    //         .map_err(|e| RenderError::CustomError(format!("Cannot render SVG scene: {e}")))?;

    //     let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //         label: Some("svg-readback-encoder"),
    //     });

    //     encoder.copy_texture_to_buffer(
    //         wgpu::TexelCopyTextureInfo {
    //             texture: &cache.texture,
    //             mip_level: 0,
    //             origin: wgpu::Origin3d::ZERO,
    //             aspect: wgpu::TextureAspect::All,
    //         },
    //         wgpu::TexelCopyBufferInfo {
    //             buffer: &cache.buffer,
    //             layout: wgpu::TexelCopyBufferLayout {
    //                 offset: 0,
    //                 bytes_per_row: Some(cache.padded_bytes_per_row),
    //                 rows_per_image: Some(height),
    //             },
    //         },
    //         wgpu::Extent3d {
    //             width,
    //             height,
    //             depth_or_array_layers: 1,
    //         },
    //     );
    //     queue.submit(Some(encoder.finish()));

    //     let slice = cache.buffer.slice(..);
    //     slice.map_async(wgpu::MapMode::Read, |_| {});

    //     device
    //         .poll(wgpu::PollType::wait_indefinitely())
    //         .map_err(|e| RenderError::CustomError(format!("Cannot wait for GPU readback: {e}")))?;

    //     {
    //         let mapped = slice.get_mapped_range();
    //         let unpadded_len = cache.unpadded_bytes_per_row as usize;
    //         let padded_len = cache.padded_bytes_per_row as usize;

    //         if unpadded_len == padded_len {
    //             output_buf[..unpadded_len * height as usize]
    //                 .copy_from_slice(&mapped[..unpadded_len * height as usize]);
    //         } else {
    //             for row in 0..height as usize {
    //                 let src_offset = row * padded_len;
    //                 let dst_offset = row * unpadded_len;
    //                 output_buf[dst_offset..dst_offset + unpadded_len]
    //                     .copy_from_slice(&mapped[src_offset..src_offset + unpadded_len]);
    //             }
    //         }
    //     }

    //     cache.buffer.unmap();

    //     Ok(())
    // }
}
