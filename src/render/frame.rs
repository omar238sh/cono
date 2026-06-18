use image::{Rgba, RgbaImage};
use vello::peniko::Color;

use crate::render::{error::RenderError, models::Size};

pub struct Frame {
    pixmap: Pixmap,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self {
            pixmap: Pixmap::new(size.width, size.height).unwrap(),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.pixmap.fill(color);
    }
}

pub struct Pixmap {
    image: RgbaImage,
}

impl Pixmap {
    pub fn new(width: u32, height: u32) -> Option<Self> {
        Some(Self {
            image: RgbaImage::new(width, height),
        })
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn save_png(&self, path: impl AsRef<std::path::Path>) -> Result<(), image::ImageError> {
        self.image.save(path)
    }

    pub fn fill(&mut self, color: Color) {
        let rgba = color.to_rgba8();
        let pixel = Rgba([rgba.r, rgba.g, rgba.b, rgba.a]);
        for p in self.image.pixels_mut() {
            *p = pixel;
        }
    }

    pub fn blit_from(&mut self, src: &Pixmap, x: u32, y: u32) {
        for row in 0..src.height() {
            for col in 0..src.width() {
                let tx = x + col;
                let ty = y + row;
                if tx >= self.width() || ty >= self.height() {
                    continue;
                }
                let src_px = src.image.get_pixel(col, row);
                self.image.put_pixel(tx, ty, *src_px);
            }
        }
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        self.image.as_mut()
    }
}

pub trait Component {
    fn render(&self) -> &Pixmap;
    fn render_to(&self, target: &mut Pixmap) {
        let content = self.render();
        target.blit_from(content, 0, 0);
    }

    fn content_mut(&mut self) -> &mut Pixmap;

    fn size(&self) -> Size {
        let content = self.render();
        Size {
            width: content.width(),
            height: content.height(),
        }
    }

    fn set_color(&mut self, color: Color) -> Result<(), RenderError> {
        let rgba = color.to_rgba8();
        let buffer = self.content_mut().data_mut();
        for px in buffer.chunks_exact_mut(4) {
            let old_alpha = px[3] as u16;
            let color_alpha = rgba.a as u16;
            let new_alpha = ((old_alpha * color_alpha) / 255) as u8;
            px[0] = rgba.r;
            px[1] = rgba.g;
            px[2] = rgba.b;
            px[3] = new_alpha;
        }
        Ok(())
    }

    fn set_opacity(&mut self, alpha: f32) {
        let alpha = alpha.clamp(0.0, 1.0);
        let buffer = self.content_mut().data_mut();
        for px in buffer.chunks_exact_mut(4) {
            px[3] = (px[3] as f32 * alpha) as u8;
        }
    }
}
