use std::any::Any;
use std::sync::{LazyLock, Mutex};

use image::{ImageBuffer, Rgba};
use taffy::{NodeId, TaffyTree};
use vello::Scene;

use crate::render::components::SceneGpuRenderer;
use crate::render::components::freebox::{FreeBox, LocatedComponent};
use crate::render::components::placeholder::{ElementPointer, PlaceHolder};
use crate::render::error::RenderError;
use crate::render::models::Size;

pub struct Frame {
    pub content: FreeBox,
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self {
            content: FreeBox::new(size),
        }
    }

    pub fn add<C: Component + 'static>(mut self, c: C) -> Self {
        self.content = self.content.with_child(c.locate());
        self
    }

    pub fn to_image(&mut self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, RenderError> {
        let mut tree = TaffyTree::new();
        self.content.register_layout(&mut tree)?;
        let mut renderer = SceneGpuRenderer::new()?;
        let image = renderer.render_to_image(
            &self.content.render(&tree)?,
            self.content.size()?.width as u32,
            self.content.size()?.height as u32,
        )?;
        Ok(image)
    }
}

// static Tree: LazyLock<Mutex<TaffyTree>> = LazyLock::new(|| Mutex::new(TaffyTree::new()));

pub trait Component: ComponentClone + Send + 'static {
    fn render(&self, tree: &TaffyTree) -> Result<Scene, RenderError>;

    fn size(&self) -> Result<Size, RenderError>;

    fn computed_size(&self, tree: &TaffyTree) -> Result<Size, RenderError> {
        let id = self.layout()?;
        let layout = tree.layout(id)?;

        let size = Size::new(layout.size.width as f64, layout.size.height as f64);

        Ok(size)
    }

    fn register_layout(&mut self, tree: &mut TaffyTree) -> Result<NodeId, RenderError>;
    fn layout(&self) -> Result<NodeId, RenderError>;

    fn locate(self) -> LocatedComponent
    where
        Self: Sized + 'static,
    {
        LocatedComponent::new(self)
    }

    fn to_placeholder(self) -> Result<PlaceHolder<Self>, RenderError>
    where
        Self: Sized + Clone,
    {
        PlaceHolder::from(self)
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
    T: Component + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
