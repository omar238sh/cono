use tiny_skia::Pixmap;

use crate::render::{
    components::Component,
    error::RenderError,
    models::{Rgba, Size},
};

pub struct Frame {
    pixmap: Pixmap,
    components: Vec<Box<dyn Component>>,
}

impl Frame {
    pub fn new(size: Size) -> Result<Self, RenderError> {
        let pixmap = Pixmap::new(size.width, size.height).ok_or(RenderError::CannotCreatePixmap)?;

        Ok(Self {
            pixmap,
            components: Vec::new(),
        })
    }

    pub fn set_background(&mut self, color: Rgba) {
        self.pixmap.fill(color.to_tiny_skia());
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }

    pub fn render_components(&mut self) {
        for component in &self.components {
            component.render_to(&mut self.pixmap);
        }
    }

    pub fn pixmap(&self) -> &Pixmap {
        &self.pixmap
    }
}
