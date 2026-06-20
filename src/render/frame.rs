use vello::Scene;
use vello::peniko::Color;
use vello::kurbo::{Affine, Rect};

use crate::render::models::Size;

pub struct Frame {
    pub scene: Scene,
    pub size: Size,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self {
            scene: Scene::new(),
            size,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        let rect = Rect::new(0.0, 0.0, self.size.width as f64, self.size.height as f64);
        self.scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            color,
            None,
            &rect,
        );
    }
}

pub trait Component {
    fn render(&self) -> Scene;
    
    fn render_to(&self, target: &mut Scene, transform: Affine) {
        let content = self.render();
        target.append(&content, Some(transform));
    }

    fn size(&self) -> Size;
}
