use tiny_skia::Pixmap;

use crate::render::models::{Bounds, Rgba};

pub trait Component {
    fn id(&self) -> &str;
    fn bounds(&self) -> Bounds;
    fn render_to(&self, target: &mut Pixmap);
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub id: String,
    pub bounds: Bounds,
    pub fill: Rgba,
}

impl Rectangle {
    pub fn new(id: impl Into<String>, bounds: Bounds, fill: Rgba) -> Self {
        Self {
            id: id.into(),
            bounds,
            fill,
        }
    }
}

impl Component for Rectangle {
    fn id(&self) -> &str {
        &self.id
    }

    fn bounds(&self) -> Bounds {
        self.bounds
    }

    fn render_to(&self, target: &mut Pixmap) {
        let color = self.fill.to_tiny_skia().premultiply().to_color_u8();
        let width = target.width();
        let height = target.height();

        let start_x = self.bounds.position.x.clamp(0, width as i32) as u32;
        let start_y = self.bounds.position.y.clamp(0, height as i32) as u32;

        let end_x = self
            .bounds
            .position
            .x
            .saturating_add(self.bounds.size.width as i32)
            .clamp(0, width as i32) as u32;
        let end_y = self
            .bounds
            .position
            .y
            .saturating_add(self.bounds.size.height as i32)
            .clamp(0, height as i32) as u32;

        if start_x >= end_x || start_y >= end_y {
            return;
        }

        let row_width = width as usize;
        let pixels = target.pixels_mut();

        for py in start_y..end_y {
            let row = py as usize * row_width;
            for px in start_x..end_x {
                pixels[row + px as usize] = color;
            }
        }
    }
}
