use vello::Scene;

use crate::render::{frame::Component, models::Size};

pub struct Column {
    contents: Vec<Box<dyn Component>>,
    size: Size,
}

impl Column {
    pub fn new(size: Size) -> Self {
        Self {
            contents: Vec::new(),
            size: size,
        }
    }
}

impl Component for Column {
    fn render(&self) -> Scene {
        todo!()
    }

    fn size(&self) -> Size {
        self.size
    }
}
