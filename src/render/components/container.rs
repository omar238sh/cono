use std::ops::Deref;

use taffy::{JustifyContent, NodeId, Style, TaffyTree, prelude::length};
use vello::{
    Scene,
    kurbo::{Affine, Rect},
    peniko::Color,
};

use crate::render::{
    components::style::Padding, error::RenderError, frame::Component, models::Size,
};

#[derive(Clone)]
pub struct Container {
    pub id: Option<NodeId>,
    pub child: Box<dyn Component>,
    pub size: Size,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: f64,
    pub border_radius: f64,
    pub padding: Padding,
}

impl Deref for Container {
    type Target = dyn Component;

    fn deref(&self) -> &Self::Target {
        &*self.child
    }
}

impl Container {
    pub fn new(content: impl Component + 'static) -> Result<Self, RenderError> {
        let size = content.size()?;
        Ok(Self {
            id: None,
            child: Box::new(content),
            size: size,
            background_color: None,
            border_color: None,
            border_width: 0f64,
            border_radius: 0f64,
            padding: Padding::new(0f64),
        })
    }

    pub fn from_size(size: Size) -> Self {
        Self::new(PhantomComponent::new(size)).unwrap()
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn border_width(mut self, width: f64) -> Self {
        self.border_width = width;
        self
    }

    pub fn border_radius(mut self, radius: f64) -> Self {
        self.border_radius = radius;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn set_background(&mut self, color: Color) {
        self.background_color = Some(color);
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = Some(color);
    }

    pub fn set_border_width(&mut self, width: f64) {
        self.border_width = width;
    }

    pub fn set_border_radius(&mut self, radius: f64) {
        self.border_radius = radius;
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }
}

impl Component for Container {
    fn render(&self, tree: &TaffyTree) -> Result<vello::Scene, crate::render::error::RenderError> {
        let child_id = tree.layout(self.child.layout()?)?;

        let mut scene = Scene::new();
        let size = self.computed_size(tree)?;
        let rect = Rect::new(0.0, 0.0, size.width, size.height).to_rounded_rect(self.border_radius);
        if let Some(background_color) = self.background_color {
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                background_color,
                None,
                &rect,
            );
        }

        let has_clip = self.border_radius > 0.0;
        if has_clip {
            scene.push_layer(
                vello::peniko::Fill::NonZero,
                vello::peniko::Mix::Normal,
                1.0,
                Affine::IDENTITY,
                &rect,
            );
        }

        scene.append(
            &self.child.render(tree)?,
            Some(Affine::translate((
                child_id.location.x as f64,
                child_id.location.y as f64,
            ))),
        );

        if has_clip {
            scene.pop_layer();
        }

        if let Some(border_color) = self.border_color {
            if self.border_width > 0.0 {
                let stroke = vello::kurbo::Stroke::new(self.border_width);
                let half_bw = self.border_width / 2.0;
                let stroke_rect = Rect::new(
                    half_bw,
                    half_bw,
                    size.width - half_bw,
                    size.height - half_bw,
                )
                .to_rounded_rect((self.border_radius - half_bw).max(0.0));
                scene.stroke(&stroke, Affine::IDENTITY, border_color, None, &stroke_rect);
            }
        }

        Ok(scene)
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        Ok(self.size)
    }

    fn register_layout(
        &mut self,
        tree: &mut TaffyTree,
    ) -> std::result::Result<taffy::NodeId, RenderError> {
        if let Some(node_id) = self.id {
            return Ok(node_id);
        }
        let style = taffy::Style {
            size: taffy::Size {
                width: length(self.size()?.width as f32),
                height: length(self.size()?.height as f32),
            },
            display: taffy::Display::Flex,
            align_items: Some(taffy::AlignItems::CENTER),
            padding: self.padding.to_rect(),
            justify_content: Some(JustifyContent::CENTER),
            ..Default::default()
        };

        let child_layout = self.child.register_layout(tree)?;

        let id = tree.new_with_children(style, &[child_layout])?;
        self.id = Some(id);
        tree.compute_layout(id, taffy::Size::max_content())?;
        Ok(id)
    }

    fn layout(&self) -> Result<taffy::NodeId, crate::render::error::RenderError> {
        match self.id {
            Some(v) => return Ok(v),
            None => return Err(crate::render::error::RenderError::NotComputed),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct PhantomComponent {
    size: Size,
    id: Option<NodeId>,
}

impl PhantomComponent {
    pub fn new(size: Size) -> Self {
        Self { size, id: None }
    }
}

impl Component for PhantomComponent {
    fn render(&self, _: &TaffyTree) -> Result<Scene, RenderError> {
        Ok(Scene::new())
    }

    fn size(&self) -> Result<Size, RenderError> {
        Ok(self.size)
    }

    fn register_layout(&mut self, tree: &mut TaffyTree) -> Result<NodeId, RenderError> {
        let style = taffy::Style {
            size: taffy::Size {
                width: length(self.size()?.width as f32),
                height: length(self.size()?.height as f32),
            },
            display: taffy::Display::Flex,
            align_items: Some(taffy::AlignItems::CENTER),
            justify_content: Some(JustifyContent::CENTER),
            ..Default::default()
        };
        let a = tree.new_leaf(style)?;
        self.id = Some(a);
        tree.compute_layout(a, taffy::Size::max_content())?;
        Ok(a)
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

#[cfg(test)]
mod tests {
    use super::Component;
    use crate::{
        assets::Assets,
        render::{
            components::{
                SceneGpuRenderer, container::Container, row::Row, style::Arrangement, svg::SvgBox,
            },
            error::RenderError,
            models::Size,
            typst::{LuaWorld, pager::Pager},
        },
    };
    use taffy::TaffyTree;
    use vello::peniko::Color;
    #[test]
    fn test_row_2() -> Result<(), RenderError> {
        let mut tree = TaffyTree::new();
        let mut row = Row::new(Size::x_landscape())
            .arrangment(Arrangement::SpaceAround)
            .alignment(crate::render::components::style::Alignment::Center);
        let logo_svg = String::from_utf8(Assets::get("logo.svg").unwrap().data.to_vec()).unwrap();
        let text = LuaWorld::new(Pager::new().add("Cono")).to_svgs().unwrap();
        let logo = SvgBox::new(logo_svg)?.scale(2.0);

        for t in text {
            row.push(t.scale(4.0));
        }
        row.push(logo);
        let mut container = Container::new(row)?.background(Color::from_rgb8(48, 25, 52));

        container.register_layout(&mut tree)?;
        let mut renderer = SceneGpuRenderer::new()?;

        let image = renderer.render_to_image(
            &container.render(&tree)?,
            container.size()?.width as u32,
            container.size()?.height as u32,
        )?;

        image
            .save(format!("./tests/logo.png"))
            .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;

        Ok(())
    }
}
