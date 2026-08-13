use std::{
    f64::consts::PI,
    ops::{Add, Deref, DerefMut, Sub},
};

use crate::render::{error::RenderError, frame::Component, models::Size};
use taffy::{NodeId, prelude::length};
use vello::{
    Scene,
    kurbo::{Affine, Point, Rect},
    peniko::{Fill, Mix},
};

#[derive(Clone)]
pub struct FreeBox {
    pub(crate) contents: Vec<LocatedComponent>,
    pub(crate) size: Size,
    id: Option<NodeId>,
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
        pin: bool,
    },
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
            pin: false,
        }
    }

    pub fn pin(&mut self) {
        match self {
            Self::WithProgress { pin, .. } => *pin = true,
            _ => {}
        }
    }

    pub fn update_progress(&mut self, p: f64) {
        match self {
            Self::WithProgress { progress, pin, .. } => {
                if !*pin {
                    *progress = p
                } else {
                }
            }
            _ => {}
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
                ..
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

#[derive(Clone)]
pub struct Location {
    pub x: Coordinate,
    pub y: Coordinate,
    pub anchor: Anchor,
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

#[derive(Clone, Debug)]
pub enum Opacity {
    Absolute(f64),
    Progress { start: f64, end: f64, progress: f64 },
}

impl Opacity {
    pub fn new(value: f64) -> Self {
        Self::Absolute(value)
    }

    pub fn with_progress(start: f64, end: f64) -> Self {
        Self::Progress {
            start,
            end,
            progress: 0.0,
        }
    }

    pub fn update_progress(&mut self, p: f64) {
        match self {
            Opacity::Absolute(_) => {}
            Opacity::Progress { progress, .. } => *progress = p,
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::Absolute(v) => *v,
            Self::Progress {
                start,
                end,
                progress,
            } => *start + *progress * (end - start),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Angle {
    Absolute(f64),
    Progress { start: f64, end: f64, progress: f64 },
}

impl Angle {
    pub fn new(value: f64) -> Self {
        Self::Absolute(value)
    }

    pub fn with_progress(start: f64, end: f64) -> Self {
        Self::Progress {
            start,
            end,
            progress: 0.0,
        }
    }

    pub fn update_progress(&mut self, p: f64) {
        match self {
            Self::Absolute(_) => {}
            Self::Progress { progress, .. } => *progress = p,
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::Absolute(v) => *v / 180.0 * PI,
            Self::Progress {
                start,
                end,
                progress,
            } => (*start + *progress * (end - start)) / 180.0 * PI,
        }
    }
}

#[derive(Clone)]
pub struct LocatedComponent {
    pub child: Box<dyn Component>,
    pub location: Location,
    pub opacity: Opacity,
    pub rotation_anchor: Point,
    pub angle: Angle,
}

impl LocatedComponent {
    pub fn new<C: Component + 'static>(child: C) -> Self {
        let size = child.size().unwrap_or(Size::new(0.0, 0.0));
        let location = child.get_location().unwrap_or_else(Location::zero);
        let opacity = child.get_opacity().unwrap_or_else(|| Opacity::new(1.0));
        let angle = child.get_angle().unwrap_or_else(|| Angle::new(0.0));
        Self {
            child: Box::new(child),
            location,
            opacity,
            angle,
            rotation_anchor: Point::new(size.width / 2.0, size.height / 2.0),
        }
    }

    pub fn from_box(child: Box<dyn Component>) -> Self {
        let size = child.size().unwrap_or(Size::new(0.0, 0.0));
        let location = child.get_location().unwrap_or_else(Location::zero);
        let opacity = child.get_opacity().unwrap_or_else(|| Opacity::new(1.0));
        let angle = child.get_angle().unwrap_or_else(|| Angle::new(0.0));
        Self {
            child: child,
            location,
            opacity,
            angle,
            rotation_anchor: Point::new(size.width / 2.0, size.height / 2.0),
        }
    }

    pub fn sync_location(&mut self) {
        if let Some(loc) = self.child.get_location() {
            self.location = loc;
        }
        if let Some(op) = self.child.get_opacity() {
            self.opacity = op;
        }
        if let Some(ang) = self.child.get_angle() {
            self.angle = ang;
        }
    }

    pub fn set_component(&mut self, comp: impl Component) {
        self.child = Box::new(comp);
    }

    pub fn get_location(&self) -> Location {
        if let Some(loc) = self.child.get_location() {
            loc
        } else {
            self.location.clone()
        }
    }

    pub fn sync_angle(&mut self) {
        if let Some(angle) = self.child.get_angle() {
            self.angle = angle;
        }
    }

    pub fn sync_opacity(&mut self) {
        if let Some(opacity) = self.child.get_opacity() {
            self.opacity = opacity;
        }
    }

    pub fn get_angle(&self) -> Angle {
        if let Some(angle) = self.child.get_angle() {
            angle
        } else {
            self.angle.clone()
        }
    }

    pub fn get_opacity(&self) -> Opacity {
        if let Some(opacity) = self.child.get_opacity() {
            opacity
        } else {
            self.opacity.clone()
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

    pub fn opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn angle(mut self, angle: Angle) -> Self {
        self.angle = angle;
        self
    }

    pub fn set_location(&mut self, location: Location) {
        self.location = location;
    }

    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.location.anchor = anchor;
    }

    pub fn update_progress(&mut self, progress: f64) {
        self.location.update_progress(progress);
        self.opacity.update_progress(progress);
        self.angle.update_progress(progress);
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
        self.sync_angle();
        self.sync_opacity();
        self.child.register_layout(tree)
    }

    fn layout(&self) -> Result<NodeId, RenderError> {
        self.child.layout()
    }

    fn get_location(&self) -> Option<Location> {
        Some(self.get_location())
    }

    fn get_angle(&self) -> Option<Angle> {
        Some(self.get_angle())
    }

    fn get_opacity(&self) -> Option<Opacity> {
        Some(self.get_opacity())
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

    pub fn with_progress(loc1: Self, loc2: Self) -> Self {
        let loc = Self::new(
            Coordinate::with_progress(loc1.x, loc2.x),
            Coordinate::with_progress(loc1.y, loc2.y),
        );
        loc
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

    pub fn update_progress(&mut self, p: f64) {
        self.x.update_progress(p);
        self.y.update_progress(p);
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
            let Location { x, y, .. } = component.get_location().as_topleft(child_size);
            let opacity = component.get_opacity();
            let angle = component.get_angle();
            let anchor_point = Point::new(child_size.width / 2.0, child_size.height / 2.0);
            let transform =
                Affine::translate((x.to_num(self.size.width), y.to_num(self.size.height)))
                    * Affine::rotate_about(angle.value(), anchor_point);
            let rect = Rect::new(0.0, 0.0, child_size.width, child_size.height);

            scene.push_layer(
                Fill::NonZero,
                Mix::Normal,
                opacity.value() as f32,
                transform,
                &rect,
            );

            let s = component.render(tree)?;
            scene.append(&s, Some(transform));

            scene.pop_layer();
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

    #[test]
    fn test_freebox_rotation_and_opacity() -> Result<(), RenderError> {
        use super::{Angle, Opacity};
        let mut tree = TaffyTree::new();
        let text = Text::new("Rotated & Faded").color("#FFFFFF").size(60);
        let located = LocatedComponent::new(text)
            .location(Location::center(Anchor::Center))
            .opacity(Opacity::new(0.5))
            .angle(Angle::new(45.0));

        let mut freebox = FreeBox::new(Size::youtube_video()).with_child(located);
        freebox.register_layout(&mut tree)?;

        let mut renderer = SceneGpuRenderer::new()?;
        let image = renderer.render_to_image(
            &freebox.render(&tree)?,
            freebox.size()?.width as u32,
            freebox.size()?.height as u32,
        )?;
        image.save("./tests/rotate.png").unwrap();
        assert_eq!(image.width(), 1920);
        assert_eq!(image.height(), 1080);
        Ok(())
    }
}
