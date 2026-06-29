use taffy::{NodeId, TaffyResult, TaffyTree};
use vello::Scene;
use vello::kurbo::{Affine, Rect};
use vello::peniko::Color;

use crate::render::error::RenderError;
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
    fn render(&self, tree: &TaffyTree) -> Result<Scene, RenderError>;

    fn size(&self) -> Size;

    fn computed_size(&self, tree: &TaffyTree) -> Result<Size, RenderError> {
        let id = self.layout()?;
        let layout = tree.layout(id)?;

        let size = Size::new(layout.size.width as f64, layout.size.height as f64);

        Ok(size)
    }

    fn register_layout(&mut self, tree: &mut TaffyTree) -> TaffyResult<NodeId>;
    fn layout(&self) -> Result<NodeId, RenderError>;
}
