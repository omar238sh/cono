use taffy::{AlignItems, JustifyContent, NodeId, TaffyTree, prelude::length};
use vello::{Scene, kurbo::Affine};

use crate::render::{
    components::style::{Alignment, Arrangement, Padding, Style},
    error::RenderError,
    frame::Component,
    models::Size,
};

#[derive(Clone)]
pub struct Column {
    pub(crate) contents: Vec<Box<dyn Component>>,
    id: Option<NodeId>,
    pub(crate) style: Style,
}

impl Column {
    pub fn new(size: Size) -> Self {
        Self {
            contents: Vec::new(),
            id: None,
            style: Style::new(size),
        }
    }

    pub fn push<T: Component + 'static>(&mut self, component: T) {
        self.contents.push(Box::new(component));
    }

    pub fn arrangment(mut self, arrangement: Arrangement) -> Self {
        *self.style.arrangement_mut() = arrangement;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        *self.style.alignment_mut() = alignment;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        *self.style.padding_mut() = padding;
        self
    }

    pub fn set_arrangment(&mut self, arrangement: Arrangement) {
        *self.style.arrangement_mut() = arrangement;
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        *self.style.alignment_mut() = alignment;
    }

    pub fn set_padding(&mut self, padding: Padding) {
        *self.style.padding_mut() = padding;
    }
}

impl Component for Column {
    fn render(&self, tree: &TaffyTree) -> Result<Scene, RenderError> {
        let mut scene = Scene::new();

        for component in &self.contents {
            let layout = tree.layout(component.layout()?)?;
            scene.append(
                &component.render(tree)?,
                Some(Affine::translate((
                    layout.location.x as f64,
                    layout.location.y as f64,
                ))),
            );
        }

        Ok(scene)
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        Ok(self.style.size())
    }

    fn register_layout(
        &mut self,
        tree: &mut TaffyTree,
    ) -> std::result::Result<taffy::NodeId, RenderError> {
        if let Some(id) = self.id {
            return Ok(id);
        }
        let ids = self
            .contents
            .iter_mut()
            .map(|a| a.register_layout(tree))
            .collect::<Result<Vec<NodeId>, RenderError>>()?;

        let mut parent_style: taffy::Style = taffy::Style {
            flex_direction: taffy::FlexDirection::Column,

            size: taffy::Size {
                width: length(self.size()?.width as f32),
                height: length(self.size()?.height as f32),
            },
            ..Default::default()
        };

        match self.style.arrangement() {
            Arrangement::Start => parent_style.justify_content = Some(JustifyContent::START),
            Arrangement::End => parent_style.justify_content = Some(JustifyContent::END),
            Arrangement::Center => parent_style.justify_content = Some(JustifyContent::CENTER),
            Arrangement::SpaceBetween => {
                parent_style.justify_content = Some(JustifyContent::SPACE_BETWEEN)
            }
            Arrangement::SpaceAround => {
                parent_style.justify_content = Some(JustifyContent::SPACE_AROUND)
            }
            Arrangement::SpaceEvenly => {
                parent_style.justify_content = Some(JustifyContent::SPACE_EVENLY)
            }
            Arrangement::SpacedBy(pixels) => {
                parent_style.justify_content = Some(JustifyContent::START);
                parent_style.gap.height = length(pixels.clone() as f32);
            }
        }

        match self.style.alignment() {
            super::style::Alignment::Start => parent_style.align_items = Some(AlignItems::START),
            super::style::Alignment::End => parent_style.align_items = Some(AlignItems::END),
            super::style::Alignment::Center => parent_style.align_items = Some(AlignItems::CENTER),
        }

        parent_style.padding = self.style.padding().to_rect();

        let id = tree.new_with_children(parent_style, &ids)?;
        tree.compute_layout(id, taffy::Size::max_content())?;
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

#[cfg(test)]
mod tests {
    use taffy::TaffyTree;

    use crate::render::{
        components::{SceneGpuRenderer, column::Column, text::Text},
        error::RenderError,
        frame::Component,
        models::Size,
    };
    #[test]
    fn test_column() -> Result<(), RenderError> {
        let mut tree = TaffyTree::new();
        let mut column = Column::new(Size::tiktok())
            .arrangment(super::Arrangement::SpaceAround)
            .alignment(crate::render::components::style::Alignment::Center);

        column.push(Text::new("Hello").color("#87d5fe"));
        column.register_layout(&mut tree)?;

        let mut renderer = SceneGpuRenderer::new()?;

        let image = renderer.render_to_image(
            &column.render(&tree)?,
            column.size()?.width as u32,
            column.size()?.height as u32,
        )?;

        image
            .save(format!("./tests/column.png"))
            .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;

        Ok(())
    }
}
