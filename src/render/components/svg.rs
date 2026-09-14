use std::{fs::read_to_string, path::Path};

use taffy::{NodeId, Style, TaffyTree, prelude::length};
use vello::{Scene, kurbo::Affine};
use vello_svg::{self as vsvg, animate::append_animated, animation::AnimationConfig};

use crate::render::{
    components::style::Padding, error::RenderError, frame::Component, models::Size,
};

#[derive(Clone)]
pub struct SvgBox {
    // pub(crate) scene: Scene,
    pub(crate) size: Size,
    pub(crate) padding: Padding,
    pub(crate) progress: f64,
    pub(crate) config: AnimationConfig,
    content: String,
    resize: Option<Size>,
    scale: Option<f64>,
    pin: bool,
    id: Option<NodeId>,
}

impl Component for SvgBox {
    fn render(&self, _: &TaffyTree) -> Result<Scene, RenderError> {
        let mut scene = Scene::new();
        append_animated(&mut scene, &self.content, self.progress, &self.config)?;
        dbg!(&self.progress);
        if let Some(new_size) = self.resize {
            (scene, _) = with_size(&scene, &self.size, new_size);
        }

        if let Some(factor) = self.scale {
            (scene, _) = scale(&scene, self.size, factor)
        }

        Ok(scene.clone())
    }

    fn pin(&mut self) {
        self.pin = true;
    }

    fn unpin(&mut self) {
        self.pin = false;
    }

    fn update_child_progress(&mut self, _progress: f64) {
        self.set_progress(_progress);
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        let mut size = self.size.clone();

        if let Some(new_size) = self.resize {
            size = new_size;
        }

        if let Some(factor) = self.scale {
            size = Size::new(size.width * factor, size.height * factor);
        }

        Ok(size)
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
            margin: self.padding.to_rect(),
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

        // let scene = vsvg::render_tree(&tree);

        Ok(Self {
            pin: false,
            padding: Padding::new(0.0),
            size: Size::new(width, height),
            id: None,
            progress: 0f64,
            // tree,
            resize: None,
            scale: None,
            config: AnimationConfig::draw(),
            content,
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

        Ok(Self {
            padding: Padding::new(0.0),
            size: Size::new(width, height),
            id: None,
            progress: 0.0,
            resize: None,
            scale: None,
            config: AnimationConfig::draw(),
            content,
            pin: false,
        })
    }

    pub fn set_progress(&mut self, progress: f64) {
        if !self.pin {
            self.progress = progress
        };
    }

    pub fn with_size(mut self, new_size: Size) -> Self {
        self.resize = Some(new_size);
        self
    }

    pub fn scale(mut self, factor: f64) -> Self {
        self.scale = Some(factor);
        self
    }

    pub fn set_scale(&mut self, factor: f64) {
        self.scale = Some(factor);
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }
}

pub fn with_size(scene: &Scene, size: &Size, new_size: Size) -> (Scene, Size) {
    let mut new_scene = Scene::new();
    let height_factor = new_size.height / size.height;
    let width_factor = new_size.width / size.width;
    new_scene.append(
        &scene,
        Some(Affine::scale_non_uniform(
            width_factor as f64,
            height_factor as f64,
        )),
    );
    (new_scene, new_size)
}

pub fn scale(scene: &Scene, size: Size, factor: f64) -> (Scene, Size) {
    let mut new_scene = Scene::new();
    new_scene.append(scene, Some(Affine::scale(factor)));
    let new_size = Size::new(size.width * factor, size.height * factor);
    (new_scene, new_size)
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
