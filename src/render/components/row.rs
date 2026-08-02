use taffy::{AlignItems, JustifyContent, NodeId, TaffyError, TaffyTree, prelude::length};
use vello::{Scene, kurbo::Affine};

use crate::{
    down_cast,
    render::{
        components::style::{Alignment, Arrangement, Padding, Style},
        error::RenderError,
        frame::Component,
        models::Size,
    },
};

#[derive(Clone)]
pub struct Row {
    contents: Vec<Box<dyn Component>>,
    id: Option<NodeId>,
    style: Style,
}

impl mlua::FromLua for Row {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Row".to_string(),
            message: Some("expected Row UserData".to_string()),
        })
    }
}

impl Row {
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

impl Component for Row {
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
            flex_direction: taffy::FlexDirection::Row,

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
                parent_style.gap.width = length(pixels.clone() as f32);
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
impl mlua::UserData for Row {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.style.size()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, size: Size| Ok(Row::new(size)));
        methods.add_method_mut("push", |_, this, val: mlua::AnyUserData| {
            down_cast!(val, |a| this.contents.push(Box::new(a)));
            Ok(())
        });
        methods.add_method("arrangment", |_, this, arr: Arrangement| {
            Ok(this.clone().arrangment(arr))
        });
        methods.add_method_mut("set_arrangment", |_, this, arr: Arrangement| {
            this.set_arrangment(arr);
            Ok(())
        });
        methods.add_method("alignment", |_, this, align: Alignment| {
            Ok(this.clone().alignment(align))
        });
        methods.add_method_mut("set_alignment", |_, this, align: Alignment| {
            this.set_alignment(align);
            Ok(())
        });
        methods.add_method("padding", |_, this, pad: Padding| {
            Ok(this.clone().padding(pad))
        });
        methods.add_method_mut("set_padding", |_, this, pad: Padding| {
            this.set_padding(pad);
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use taffy::TaffyTree;
    use vello::peniko::Color;

    use crate::render::{
        components::{
            SceneGpuRenderer, column::Column, container::Container, row::Row, style::Padding,
            svg::SvgBox,
        },
        error::RenderError,
        frame::Component,
        models::Size,
        typst::{LuaWorld, pager::Pager},
    };

    #[test]
    fn test_row() -> Result<(), RenderError> {
        let mut tree = TaffyTree::new();
        let mut row = Row::new(Size::x_landscape())
            .arrangment(super::Arrangement::SpaceAround)
            .alignment(crate::render::components::style::Alignment::Center);

        let world = LuaWorld::new(Pager::new().add("Hello Row").add("مرحبا").add("عمر يفعلها"))
            .to_svgs()
            .map_err(|_| RenderError::CustomError("Cannot make svgs".to_string()))?
            .into_iter()
            .map(|a| {
                let svg = a.scale(3.0);
                Ok(svg)
            })
            .collect::<Result<Vec<SvgBox>, RenderError>>()?;

        for text in world {
            row.push(text);
        }

        row.register_layout(&mut tree)?;
        let mut renderer = SceneGpuRenderer::new()?;

        let image = renderer.render_to_image(
            &row.render(&tree)?,
            row.size()?.width as u32,
            row.size()?.height as u32,
        )?;

        image
            .save(format!("./tests/row.png"))
            .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;

        Ok(())
    }

    #[test]
    fn test_row_column_compose() -> Result<(), RenderError> {
        let mut tree = TaffyTree::new();
        let mut column = Column::new(Size::tiktok())
            .arrangment(super::Arrangement::SpaceEvenly)
            .alignment(crate::render::components::style::Alignment::Center);

        let svgs = LuaWorld::new(
            Pager::new()
                .add("Title: Composition Test")
                .add("Col Item 1")
                .add("Row Item A")
                .add("Row Item B")
                .add("Col Item 2"),
        )
        .to_svgs()
        .map_err(|_| RenderError::CustomError("Cannot make svgs".to_string()))?
        .into_iter()
        .map(|a| {
            let svg = a.scale(3.0);
            let c = Container::new(svg)?
                .padding(Padding::new(5.0))
                .background(Color::from_rgba8(150, 255, 100, 100))
                .border_radius(10.0);
            Ok(c)
        })
        .collect::<Result<Vec<Container>, RenderError>>()?;

        let mut iter = svgs.into_iter();

        if let Some(item) = iter.next() {
            column.push(item);
        }

        if let Some(item) = iter.next() {
            column.push(item);
        }

        let mut row = Row::new(Size::new(900.0, 300.0))
            .arrangment(super::Arrangement::SpaceAround)
            .alignment(crate::render::components::style::Alignment::Center);

        if let Some(item) = iter.next() {
            row.push(item);
        }
        if let Some(item) = iter.next() {
            row.push(item);
        }

        column.push(row);

        if let Some(item) = iter.next() {
            column.push(item);
        }

        column.register_layout(&mut tree)?;
        let mut renderer = SceneGpuRenderer::new()?;

        let image = renderer.render_to_image(
            &column.render(&tree)?,
            column.size()?.width as u32,
            column.size()?.height as u32,
        )?;

        image
            .save(format!("./tests/compose.png"))
            .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;

        Ok(())
    }
}
