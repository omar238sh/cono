use tiny_skia::{Color, Pixmap};
use usvg::{Options, Transform, Tree};

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
    fn render(&self) -> Pixmap;
    fn render_to(&self, target: &mut Pixmap);
    fn render_region(&self, rect: tiny_skia::Rect) -> Pixmap;

    fn size(&self) -> Size;
    fn set_color(&mut self, color: Color);
    fn set_opacity(&mut self, alpha: f32);
    fn color(&self) -> Color;
}
