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
        let x = self.bounds.position.x.max(0) as u32;
        let y = self.bounds.position.y.max(0) as u32;
        let max_x = (x + self.bounds.size.width).min(target.width());
        let max_y = (y + self.bounds.size.height).min(target.height());

        let width = target.width() as usize;
        let pixels = target.pixels_mut();

        for py in y..max_y {
            let row = py as usize * width;
            for px in x..max_x {
                let idx = row + px as usize;
                pixels[idx] = color;
            }
        }
    }
}
