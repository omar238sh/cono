use tiny_skia::{Color, Paint, Pixmap};
use usvg::Transform;

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

    pub fn set_color(&mut self, color: tiny_skia::Color) {
        self.pixmap.fill(color);
    }
}

pub trait Component {
    fn render(&self) -> &Pixmap;
    fn render_to(&self, target: &mut Pixmap) {
        let paint = tiny_skia::PixmapPaint::default();
        let content = self.render();
        target.draw_pixmap(0, 0, content.as_ref(), &paint, Transform::identity(), None);
    }

    fn content_mut(&mut self) -> &mut Pixmap;

    fn size(&self) -> Size {
        let content = self.render();
        Size {
            width: content.width() as u32,
            height: content.height() as u32,
        }
    }

    fn set_color(&mut self, color: Color) -> Result<(), RenderError> {
        let mut paint = Paint::default();
        paint.set_color(color);
        paint.blend_mode = tiny_skia::BlendMode::SourceIn;

        let rect = tiny_skia::Rect::from_xywh(
            0.0,
            0.0,
            self.size().width as f32,
            self.size().height as f32,
        )
        .ok_or(RenderError::CustomError("Cannot create rect".to_string()))?;

        self.content_mut()
            .fill_rect(rect, &paint, Transform::identity(), None);
        Ok(())
    }
    fn set_opacity(&mut self, alpha: f32) {
        let pixmap = self.content_mut();

        let alpha = alpha.clamp(0.0, 1.0);

        for pixel in pixmap.pixels_mut() {
            let c = pixel.demultiply();
            let new_alpha = (c.alpha() as f32 * alpha) as u8;

            *pixel = Color::from_rgba8(pixel.red(), pixel.green(), pixel.blue(), new_alpha)
                .premultiply()
                .to_color_u8();
        }
    }
}
