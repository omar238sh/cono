use std::ops::Deref;

use taffy::{JustifyContent, NodeId, TaffyTree, prelude::length};
use vello::{
    Scene,
    kurbo::{Affine, Rect},
    peniko::Color,
};

use crate::{
    down_cast,
    render::{components::style::Padding, error::RenderError, frame::Component, models::Size},
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

impl mlua::FromLua for Container {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Container".to_string(),
            message: Some("expected Container UserData".to_string()),
        })
    }
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
        let rect = Rect::new(0.0, 0.0, 0.0 + size.width, 0.0 + size.height)
            .to_rounded_rect(self.border_radius);
        if let Some(background_color) = self.background_color {
            scene.fill(
                vello::peniko::Fill::NonZero,
                Affine::IDENTITY,
                background_color,
                None,
                &rect,
            );
        }

        if let Some(border_color) = self.border_color {
            let stroke = vello::kurbo::Stroke::new(self.border_width);
            scene.stroke(&stroke, Affine::IDENTITY, border_color, None, &rect);
        }

        scene.append(
            &self.child.render(tree)?,
            Some(Affine::translate((
                child_id.location.x as f64,
                child_id.location.y as f64,
            ))),
        );

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

fn parse_color_from_lua(val: mlua::Value) -> mlua::Result<Color> {
    match val {
        mlua::Value::String(s) => {
            let str_val = s.to_str()?;
            let trimmed = str_val.trim();
            if trimmed.starts_with('#') {
                let hex = &trimmed[1..];
                if hex.len() == 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16).map_err(mlua::Error::external)?;
                    let g = u8::from_str_radix(&hex[2..4], 16).map_err(mlua::Error::external)?;
                    let b = u8::from_str_radix(&hex[4..6], 16).map_err(mlua::Error::external)?;
                    Ok(Color::from_rgb8(r, g, b))
                } else if hex.len() == 8 {
                    let r = u8::from_str_radix(&hex[0..2], 16).map_err(mlua::Error::external)?;
                    let g = u8::from_str_radix(&hex[2..4], 16).map_err(mlua::Error::external)?;
                    let b = u8::from_str_radix(&hex[4..6], 16).map_err(mlua::Error::external)?;
                    let a = u8::from_str_radix(&hex[6..8], 16).map_err(mlua::Error::external)?;
                    Ok(Color::from_rgba8(r, g, b, a))
                } else {
                    Err(mlua::Error::external(format!(
                        "Invalid hex color: {}",
                        str_val
                    )))
                }
            } else {
                Err(mlua::Error::external(format!(
                    "Invalid color string: {}",
                    str_val
                )))
            }
        }
        mlua::Value::Table(t) => {
            let r: u8 = t.get(1).unwrap_or(0);
            let g: u8 = t.get(2).unwrap_or(0);
            let b: u8 = t.get(3).unwrap_or(0);
            let a: u8 = t.get(4).unwrap_or(255);
            Ok(Color::from_rgba8(r, g, b, a))
        }
        _ => Err(mlua::Error::external("Invalid color format")),
    }
}

impl mlua::UserData for Container {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.size));
        fields.add_field_method_set("size", |_, this, size: Size| {
            this.size = size;
            Ok(())
        });
        fields.add_field_method_get("border_width", |_, this| Ok(this.border_width));
        fields.add_field_method_set("border_width", |_, this, w: f64| {
            this.border_width = w;
            Ok(())
        });
        fields.add_field_method_get("border_radius", |_, this| Ok(this.border_radius));
        fields.add_field_method_set("border_radius", |_, this, r: f64| {
            this.border_radius = r;
            Ok(())
        });
        fields.add_field_method_get("padding", |_, this| Ok(this.padding));
        fields.add_field_method_set("padding", |_, this, p: Padding| {
            this.padding = p;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, child: mlua::Value| {
            let a = down_cast!(child, |c| {
                Container::new(c).map_err(|e| mlua::Error::external(format!("{:?}", e)))
            })
            .unwrap();
            a
        });

        methods.add_method("background", |_, this, color_val: mlua::Value| {
            let col = parse_color_from_lua(color_val)?;
            Ok(this.clone().background(col))
        });
        methods.add_method_mut("set_background", |_, this, color_val: mlua::Value| {
            let col = parse_color_from_lua(color_val)?;
            this.set_background(col);
            Ok(())
        });

        methods.add_method("border_color", |_, this, color_val: mlua::Value| {
            let col = parse_color_from_lua(color_val)?;
            Ok(this.clone().border_color(col))
        });
        methods.add_method_mut("set_border_color", |_, this, color_val: mlua::Value| {
            let col = parse_color_from_lua(color_val)?;
            this.set_border_color(col);
            Ok(())
        });

        methods.add_method("border_width", |_, this, width: f64| {
            Ok(this.clone().border_width(width))
        });
        methods.add_method_mut("set_border_width", |_, this, width: f64| {
            this.set_border_width(width);
            Ok(())
        });

        methods.add_method("border_radius", |_, this, radius: f64| {
            Ok(this.clone().border_radius(radius))
        });
        methods.add_method_mut("set_border_radius", |_, this, radius: f64| {
            this.set_border_radius(radius);
            Ok(())
        });

        methods.add_method("padding", |_, this, padding: Padding| {
            Ok(this.clone().padding(padding))
        });
        methods.add_method_mut("set_padding", |_, this, padding: Padding| {
            this.set_padding(padding);
            Ok(())
        });

        methods.add_method("with_size", |_, this, size: Size| {
            Ok(this.clone().with_size(size))
        });
        methods.add_method_mut("set_size", |_, this, size: Size| {
            this.set_size(size);
            Ok(())
        });
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
