use std::{fs::read_to_string, path::Path};

use taffy::{NodeId, Style, TaffyTree, prelude::length};
use vello::{Scene, kurbo::Affine};
use vello_svg::{self as vsvg};

use crate::render::{
    components::style::Padding, error::RenderError, frame::Component, models::Size,
};

#[derive(Clone)]
pub struct SvgBox {
    pub(crate) scene: Scene,
    // tree: Tree,
    pub(crate) size: Size,
    pub(crate) padding: Padding,
    id: Option<NodeId>,
}

impl Component for SvgBox {
    fn render(&self, _: &TaffyTree) -> Result<Scene, RenderError> {
        Ok(self.scene.clone())
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        Ok(self.size.clone())
    }

    fn register_layout(
        &mut self,
        tree: &mut taffy::TaffyTree,
    ) -> std::result::Result<taffy::NodeId, RenderError> {
        if let Some(id) = self.id {
            return Ok(id);
        }
        let style = Style {
            size: taffy::Size {
                width: length(self.size.width as f32),
                height: length(self.size.height as f32),
            },
            padding: self.padding.to_rect(),
            ..Default::default()
        };
        let id = tree.new_leaf(style)?;
        self.id = Some(id);
        Ok(id)
    }

    fn layout(&self) -> Result<NodeId, RenderError> {
        match self.id {
            Some(v) => Ok(v),
            None => Err(RenderError::NotComputed),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl SvgBox {
    pub fn new(content: String) -> Result<Self, RenderError> {
        let tree = vsvg::usvg::Tree::from_str(&content, &vsvg::usvg::Options::default())?;
        let width = tree.size().width().ceil() as f64;
        let height = tree.size().height().ceil() as f64;

        let scene = vsvg::render_tree(&tree);

        Ok(Self {
            scene,
            // tree,
            padding: Padding::new(0.0),
            size: Size::new(width, height),
            id: None,
        })
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn from_path(path: &Path) -> Result<Self, RenderError> {
        let content = read_to_string(path)
            .map_err(|_| RenderError::CustomError("Cannot read svg file".to_string()))?;

        let tree = vsvg::usvg::Tree::from_str(&content, &vsvg::usvg::Options::default())?;
        let width = tree.size().width().ceil() as f64;
        let height = tree.size().height().ceil() as f64;

        let scene = vsvg::render_tree(&tree);

        Ok(Self {
            scene,
            // tree,
            padding: Padding::new(0.0),
            size: Size::new(width, height),
            id: None,
        })
    }

    pub fn with_size(mut self, new_size: Size) -> Self {
        let mut new_scene = Scene::new();
        let height_factor = new_size.height / self.size.height;
        let width_factor = new_size.width / self.size.width;
        new_scene.append(
            &self.scene,
            Some(Affine::scale_non_uniform(
                width_factor as f64,
                height_factor as f64,
            )),
        );
        self.scene = new_scene;
        self.size = new_size;
        self
    }

    pub fn scale(mut self, factor: f64) -> Self {
        let mut new_scene = Scene::new();
        new_scene.append(&self.scene, Some(Affine::scale(factor)));
        self.scene = new_scene;
        self.size = Size::new(self.size.width * factor, self.size.height * factor);
        self
    }

    pub fn set_scale(&mut self, factor: f64) {
        let mut new_scene = Scene::new();
        new_scene.append(&self.scene, Some(Affine::scale(factor)));
        self.scene = new_scene;
        self.size = Size::new(self.size.width * factor, self.size.height * factor);
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }
}

#[cfg(test)]
mod cvg_test {
    use std::time::Instant;

    use taffy::TaffyTree;

    use crate::render::components::SceneGpuRenderer;
    use crate::render::{
        error::RenderError,
        frame::Component,
        typst::{LuaWorld, pager::Pager},
    };

    #[test]
    fn test_svg() -> Result<(), RenderError> {
        let tree = TaffyTree::new();
        let content = LuaWorld::new(
            Pager::new()
                .add("مرحبا")
                .add("Hello")
                .add("eee")
                .add("OOO")
                .add("مرحبا بالعالم"),
        )
        .to_svgs()
        .map_err(|_| RenderError::CustomError("cannot create svgs".to_string()))?;

        let mut renderer = SceneGpuRenderer::new()?;

        for (index, svg) in content.into_iter().enumerate() {
            let mysvg_box = svg;
            let p = mysvg_box.render(&tree)?;
            let size = mysvg_box.size()?;
            let start_time = Instant::now();
            let image = renderer.render_to_image(&p, size.width as u32, size.height as u32)?;
            let end = start_time.elapsed();
            println!("{} : {}us", index, end.as_micros());
            image
                .save(format!("./tests/svg_{}.png", index))
                .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;
        }
        Ok(())
    }
}
