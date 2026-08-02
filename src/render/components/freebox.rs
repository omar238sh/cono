use std::ops::{Add, Deref, DerefMut, Sub};

use crate::{
    down_cast,
    render::{error::RenderError, frame::Component, models::Size},
};
use taffy::{NodeId, prelude::length};
use vello::{Scene, kurbo::Affine};

#[derive(Clone)]
pub struct FreeBox {
    contents: Vec<LocatedComponent>,
    size: Size,
    id: Option<NodeId>,
}

impl mlua::FromLua for FreeBox {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "FreeBox".to_string(),
            message: Some("expected FreeBox UserData".to_string()),
        })
    }
}

#[derive(Clone)]
pub enum Coordinate {
    Absolute(f64),
    Percentage(f64),
    Relative {
        target: Box<Coordinate>,
        shift: Box<Coordinate>,
    },
    MRelative {
        target: Box<Coordinate>,
        shift: Box<Coordinate>,
    },

    Scaled {
        this: Box<Coordinate>,
        factor: f64,
    },
    WithProgress {
        start: Box<Coordinate>,
        end: Box<Coordinate>,
        progress: f64,
    },
}

impl mlua::FromLua for Coordinate {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Coordinate".to_string(),
            message: Some("expected Coordinate UserData".to_string()),
        })
    }
}

impl Coordinate {
    pub fn percentage(v: f64) -> Self {
        Self::Percentage(v.clamp(0.0, 1.0))
    }

    pub fn absolute(v: f64) -> Self {
        Self::Absolute(v)
    }

    pub fn zero() -> Self {
        Self::Absolute(0.0)
    }

    pub fn scale(self, factor: f64) -> Self {
        Self::Scaled {
            this: Box::new(self),
            factor,
        }
    }

    pub fn with_progress(start: Self, end: Self) -> Self {
        Self::WithProgress {
            start: Box::new(start),
            end: Box::new(end),
            progress: 0.0,
        }
    }

    pub fn to_absolute(&self, max_length: f64) -> Self {
        match self {
            Coordinate::Absolute(c) => Self::Absolute(c.clone()),
            Coordinate::Percentage(v) => Coordinate::Absolute(v * max_length),
            Coordinate::Relative { target, shift } => {
                let v1 = match target.to_absolute(max_length) {
                    Coordinate::Absolute(v) => v,
                    _ => unreachable!(),
                };

                let v2 = match shift.to_absolute(max_length) {
                    Coordinate::Absolute(v) => v,
                    _ => unreachable!(),
                };

                Coordinate::Absolute(v1 + v2)
            }
            Coordinate::MRelative { target, shift } => {
                let v1 = match target.to_absolute(max_length) {
                    Coordinate::Absolute(v) => v,
                    _ => unreachable!(),
                };

                let v2 = match shift.to_absolute(max_length) {
                    Coordinate::Absolute(v) => v,
                    _ => unreachable!(),
                };

                Coordinate::Absolute(v1 - v2)
            }
            Coordinate::Scaled { this, factor } => {
                let v1 = match this.to_absolute(max_length) {
                    Coordinate::Absolute(v) => v,
                    _ => unreachable!(),
                };
                Coordinate::Absolute(v1 * factor)
            }
            Coordinate::WithProgress {
                start,
                end,
                progress,
            } => {
                let base = match start.to_absolute(max_length) {
                    Coordinate::Absolute(v) => v,
                    _ => unreachable!(),
                };

                let target = match end.to_absolute(max_length) {
                    Coordinate::Absolute(v) => v,
                    _ => unreachable!(),
                };

                Self::Absolute(base + progress * (target - base))
            }
        }
    }

    pub fn to_num(&self, max_length: f64) -> f64 {
        let a = match self.to_absolute(max_length) {
            Coordinate::Absolute(c) => c,
            _ => unreachable!(),
        };
        a
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Relative {
            target: Box::new(self),
            shift: Box::new(rhs),
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::MRelative {
            target: Box::new(self),
            shift: Box::new(rhs),
        }
    }
}

#[derive(Clone)]
pub enum Anchor {
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl mlua::FromLua for Anchor {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Anchor".to_string(),
            message: Some("expected Anchor UserData".to_string()),
        })
    }
}

#[derive(Clone)]
pub struct Location {
    pub x: Coordinate,
    pub y: Coordinate,
    pub anchor: Anchor,
}

impl mlua::FromLua for Location {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Location".to_string(),
            message: Some("expected Location UserData".to_string()),
        })
    }
}

impl Add for Location {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Location {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            anchor: self.anchor,
        }
    }
}

impl Sub for Location {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Location {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            anchor: Anchor::TopLeft,
        }
    }
}

#[derive(Clone)]
pub struct LocatedComponent {
    pub child: Box<dyn Component>,
    pub location: Location,
}

impl mlua::FromLua for LocatedComponent {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "LocatedComponent".to_string(),
            message: Some("expected LocatedComponent UserData".to_string()),
        })
    }
}

impl LocatedComponent {
    pub fn new<C: Component + 'static>(child: C) -> Self {
        Self {
            child: Box::new(child),
            location: Location::zero(),
        }
    }

    pub fn sync_location(&mut self) {
        if let Some(loc) = self.child.get_location() {
            self.location = loc;
        }
    }

    pub fn get_location(&self) -> Location {
        if let Some(loc) = self.child.get_location() {
            loc
        } else {
            self.location.clone()
        }
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.location.anchor = anchor;
        self
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.location.anchor = anchor;
    }

    pub fn shift(&mut self, shift: Location) {
        self.location = self.location.clone() + shift;
    }
}

impl Deref for LocatedComponent {
    type Target = dyn Component;

    fn deref(&self) -> &Self::Target {
        &*self.child
    }
}

impl DerefMut for LocatedComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.child
    }
}

impl Component for LocatedComponent {
    fn render(&self, tree: &taffy::TaffyTree) -> Result<Scene, RenderError> {
        let child_scene = self.child.render(tree)?;

        Ok(child_scene)
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        self.child.size()
    }

    fn register_layout(&mut self, tree: &mut taffy::TaffyTree) -> Result<NodeId, RenderError> {
        self.sync_location();
        self.child.register_layout(tree)
    }

    fn layout(&self) -> Result<NodeId, RenderError> {
        self.child.layout()
    }

    fn get_location(&self) -> Option<Location> {
        Some(self.get_location())
    }

    fn locate(self) -> LocatedComponent
    where
        Self: Sized + 'static,
    {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Location {
    pub fn new(x: Coordinate, y: Coordinate) -> Self {
        Self {
            x,
            y,
            anchor: Anchor::TopLeft,
        }
    }

    pub fn scale(mut self, factor: f64) -> Self {
        self.x = self.x.scale(factor);
        self.y = self.y.scale(factor);
        self
    }

    pub fn zero() -> Self {
        Self {
            x: Coordinate::Absolute(0.0),
            y: Coordinate::Absolute(0.0),
            anchor: Anchor::TopLeft,
        }
    }

    pub fn center(anchor: Anchor) -> Self {
        Location {
            x: Coordinate::Percentage(0.5),
            y: Coordinate::Percentage(0.5),
            anchor: anchor,
        }
    }

    pub fn as_topleft(&self, size: Size) -> Location {
        if matches!(self.anchor, Anchor::TopLeft) {
            return self.clone();
        }

        let x_num = self.x.clone();
        let y_num = self.y.clone();
        let w_num = size.width;
        let h_num = size.height;

        let new_x = match self.anchor {
            Anchor::TopLeft | Anchor::BottomLeft => x_num,
            Anchor::TopRight | Anchor::BottomRight => x_num + Coordinate::Absolute(-w_num),
            Anchor::Center => x_num + Coordinate::Absolute(-(w_num / 2.0)),
        };

        let new_y = match self.anchor {
            Anchor::TopLeft | Anchor::TopRight => y_num,
            Anchor::BottomLeft | Anchor::BottomRight => y_num + Coordinate::Absolute(-h_num),
            Anchor::Center => y_num + Coordinate::Absolute(-(h_num / 2.0)),
        };

        Location {
            x: new_x,
            y: new_y,
            anchor: Anchor::TopLeft,
        }
    }

    pub fn set_progress(&mut self, new_progrss: f64) -> Result<(), RenderError> {
        match &mut self.x {
            Coordinate::WithProgress { progress, .. } => {
                *progress = new_progrss;
            }
            _ => {
                return Err(RenderError::CustomError(
                    "Dosen't accept progress".to_string(),
                ));
            }
        }

        match &mut self.y {
            Coordinate::WithProgress { progress, .. } => {
                *progress = new_progrss;
            }
            _ => {
                return Err(RenderError::CustomError(
                    "Dosen't accept progress".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl FreeBox {
    pub fn new(size: Size) -> Self {
        Self {
            contents: Vec::new(),
            size: size,
            id: None,
        }
    }

    pub fn with_child(mut self, child: LocatedComponent) -> Self {
        self.contents.push(child);
        self
    }

    pub fn add(&mut self, child: impl Component) {
        self.contents.push(child.locate());
    }

    pub fn add_box<C: Component + Clone>(&mut self, child: Box<C>) {
        let c = child.as_ref().clone();
        self.contents.push(c.locate());
    }
}

impl Component for FreeBox {
    fn render(
        &self,
        tree: &taffy::TaffyTree,
    ) -> Result<vello::Scene, crate::render::error::RenderError> {
        let mut scene = Scene::new();

        for component in &self.contents {
            let child_size = component.size()?;
            let Location { x, y, .. } = component.location.as_topleft(child_size);
            scene.append(
                &component.render(tree)?,
                Some(Affine::translate((
                    x.to_num(self.size.width),
                    y.to_num(self.size.height),
                ))),
            );
        }

        Ok(scene)
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        Ok(self.size)
    }

    fn register_layout(
        &mut self,
        tree: &mut taffy::TaffyTree,
    ) -> Result<NodeId, crate::render::error::RenderError> {
        if let Some(id) = self.id {
            return Ok(id);
        }
        let ids = self
            .contents
            .iter_mut()
            .map(|a| a.register_layout(tree))
            .collect::<Result<Vec<NodeId>, RenderError>>()?;

        let parent_style: taffy::Style = taffy::Style {
            size: taffy::Size {
                width: length(self.size()?.width as f32),
                height: length(self.size()?.height as f32),
            },
            ..Default::default()
        };

        let id = tree.new_with_children(parent_style, &ids)?;
        tree.compute_layout(id, taffy::Size::max_content())?;
        self.id = Some(id);
        Ok(id)
    }

    fn layout(&self) -> Result<NodeId, crate::render::error::RenderError> {
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
impl mlua::UserData for Coordinate {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("absolute", |_, v: f64| Ok(Coordinate::absolute(v)));
        methods.add_function("percentage", |_, v: f64| Ok(Coordinate::percentage(v)));
        methods.add_function("zero", |_, ()| Ok(Coordinate::zero()));
        methods.add_function(
            "with_progress",
            |_, (start, end): (Coordinate, Coordinate)| Ok(Coordinate::with_progress(start, end)),
        );
        methods.add_method("scale", |_, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
        methods.add_method("to_absolute", |_, this, max_len: f64| {
            Ok(this.to_absolute(max_len))
        });
        methods.add_method("to_num", |_, this, max_len: f64| Ok(this.to_num(max_len)));

        methods.add_meta_method(mlua::MetaMethod::Add, |_, this, other: Coordinate| {
            Ok(this.clone() + other)
        });
        methods.add_meta_method(mlua::MetaMethod::Sub, |_, this, other: Coordinate| {
            Ok(this.clone() - other)
        });
        methods.add_meta_method(mlua::MetaMethod::Mul, |_, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(match this {
                Coordinate::Absolute(v) => format!("Coordinate::Absolute({})", v),
                Coordinate::Percentage(v) => format!("Coordinate::Percentage({})", v),
                _ => "Coordinate(...)".to_string(),
            })
        });
    }
}

impl mlua::UserData for Anchor {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("center", |_, ()| Ok(Anchor::Center));
        methods.add_function("top_left", |_, ()| Ok(Anchor::TopLeft));
        methods.add_function("top_right", |_, ()| Ok(Anchor::TopRight));
        methods.add_function("bottom_left", |_, ()| Ok(Anchor::BottomLeft));
        methods.add_function("bottom_right", |_, ()| Ok(Anchor::BottomRight));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(match this {
                Anchor::Center => "Anchor::Center".to_string(),
                Anchor::TopLeft => "Anchor::TopLeft".to_string(),
                Anchor::TopRight => "Anchor::TopRight".to_string(),
                Anchor::BottomLeft => "Anchor::BottomLeft".to_string(),
                Anchor::BottomRight => "Anchor::BottomRight".to_string(),
            })
        });
    }
}

impl mlua::UserData for Location {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x.clone()));
        fields.add_field_method_set("x", |_, this, coord: Coordinate| {
            this.x = coord;
            Ok(())
        });
        fields.add_field_method_get("y", |_, this| Ok(this.y.clone()));
        fields.add_field_method_set("y", |_, this, coord: Coordinate| {
            this.y = coord;
            Ok(())
        });
        fields.add_field_method_get("anchor", |_, this| Ok(this.anchor.clone()));
        fields.add_field_method_set("anchor", |_, this, anchor: Anchor| {
            this.anchor = anchor;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, (x, y): (Coordinate, Coordinate)| {
            Ok(Location::new(x, y))
        });
        methods.add_function("zero", |_, ()| Ok(Location::zero()));
        methods.add_function("center", |_, anchor: Anchor| Ok(Location::center(anchor)));

        methods.add_method("scale", |_, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
        methods.add_method(
            "as_topleft",
            |_, this, size: Size| Ok(this.as_topleft(size)),
        );
        methods.add_method_mut("set_progress", |_, this, p: f64| {
            this.set_progress(p)
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_meta_method(mlua::MetaMethod::Add, |_, this, other: Location| {
            Ok(this.clone() + other)
        });
        methods.add_meta_method(mlua::MetaMethod::Sub, |_, this, other: Location| {
            Ok(this.clone() - other)
        });
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok("Location(...)".to_string())
        });
    }
}

impl mlua::UserData for LocatedComponent {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("location", |_, this| Ok(this.get_location()));
        fields.add_field_method_set("location", |_, this, loc: Location| {
            this.set_location(loc);
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, val: mlua::AnyUserData| {
            let located = down_cast!(val, |c| { LocatedComponent::new(c) }).unwrap();
            Ok(located)
        });

        methods.add_method_mut("update_progress", |_, this, progress| {
            this.location
                .set_progress(progress)
                .map_err(|e| mlua::Error::external(format!("{}", e)))?;
            Ok(())
        });

        methods.add_method("location", |_, this, loc: Location| {
            Ok(this.clone().location(loc))
        });
        methods.add_method_mut("set_location", |_, this, loc: Location| {
            this.set_location(loc);
            Ok(())
        });
        methods.add_method("anchor", |_, this, anchor: Anchor| {
            Ok(this.clone().anchor(anchor))
        });
        methods.add_method_mut("set_anchor", |_, this, anchor: Anchor| {
            this.set_anchor(anchor);
            Ok(())
        });
        methods.add_method_mut("shift", |_, this, shift_loc: Location| {
            this.shift(shift_loc);
            Ok(())
        });
        methods.add_method_mut("sync_location", |_, this, ()| {
            this.sync_location();
            Ok(())
        });
        methods.add_method("get_location", |_, this, ()| Ok(this.get_location()));
    }
}

impl mlua::UserData for FreeBox {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.size));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, size: Size| Ok(FreeBox::new(size)));
        methods.add_method("with_child", |_, this, child: LocatedComponent| {
            Ok(this.clone().with_child(child))
        });
        methods.add_method_mut("add", |_, this, val: mlua::AnyUserData| {
            down_cast!(val, |a| {
                this.add(a.clone());
            });
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assets::Assets,
        render::{
            components::{
                SceneGpuRenderer,
                column::Column,
                container::Container,
                row::Row,
                style::{Alignment, Arrangement},
                svg::SvgBox,
                text::Text,
            },
            error::RenderError,
            frame::Component,
            models::Size,
        },
    };
    use taffy::TaffyTree;
    use vello::peniko::Color;
    #[test]
    fn test_freebox_compose() -> Result<(), RenderError> {
        let mut tree = TaffyTree::new();

        let logo_svg =
            String::from_utf8(Assets::get("cono_circle.svg").unwrap().data.to_vec()).unwrap();
        let logo = SvgBox::new(logo_svg)?.to_placeholder()?;
        logo.pointer().with_mut(|a| {
            a.set_scale(4f64);
        })?;
        let mut row = Row::new(Size::tiktok())
            .arrangment(Arrangement::SpaceAround)
            .alignment(Alignment::Center);
        let mut column = Column::new(Size::tiktok())
            .arrangment(Arrangement::SpaceAround)
            .alignment(Alignment::Center);
        for _ in 1..8 {
            column.push(logo.clone());
        }

        for _ in 1..5 {
            row.push(column.clone());
        }

        let c = Container::new(row)?.background(Color::from_rgb8(50, 20, 20)); // #141414        

        let located_container = LocatedComponent::new(c);

        // Create a text component to place directly in the FreeBox
        let text = Text::new("FreeBox Direct Text").color("#105010").size(52);

        let located_text = LocatedComponent::new(text).location(Location::center(Anchor::Center)); // Create the FreeBox and add components
        let mut freebox = FreeBox::new(Size::tiktok())
            .with_child(located_container)
            .with_child(located_text);

        // Register layouts
        freebox.register_layout(&mut tree)?;

        // Render to image
        let mut renderer = SceneGpuRenderer::new()?;
        let image = renderer.render_to_image(
            &freebox.render(&tree)?,
            freebox.size()?.width as u32,
            freebox.size()?.height as u32,
        )?;

        image
            .save("./tests/freebox_compose.png")
            .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;

        Ok(())
    }
}
