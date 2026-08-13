use taffy::{NodeId, Style, TaffyTree, prelude::length};
use vello::{
    Scene,
    kurbo::{
        Affine, Arc as KurboArc, BezPath, Cap, Circle as KurboCircle, Ellipse as KurboEllipse,
        Join, Line as KurboLine, ParamCurve, ParamCurveArclen, Point, Rect as KurboRect,
        RoundedRect as KurboRoundedRect, Shape as KurboShape, Stroke as KurboStroke, Vec2,
    },
    peniko::{Color as PenikoColor, Fill},
};

use crate::render::{
    components::style::Padding, error::RenderError, frame::Component, models::Size,
};

#[derive(Clone, Debug)]
pub struct StrokeStyle {
    pub color: PenikoColor,
    pub width: f64,
    pub cap: Cap,
    pub join: Join,
    pub dashes: Vec<f64>,
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            color: PenikoColor::WHITE,
            width: 2.0,
            cap: Cap::Round,
            join: Join::Round,
            dashes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FillStyle {
    pub color: PenikoColor,
    pub fill_rule: Fill,
}

impl Default for FillStyle {
    fn default() -> Self {
        Self {
            color: PenikoColor::TRANSPARENT,
            fill_rule: Fill::NonZero,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CanvasShape {
    Line {
        p1: Point,
        p2: Point,
    },
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    },
    RoundedRect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        radii: (f64, f64, f64, f64),
    },
    Circle {
        cx: f64,
        cy: f64,
        r: f64,
    },
    Ellipse {
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
    },
    Arc {
        cx: f64,
        cy: f64,
        r: f64,
        start_angle_rad: f64,
        sweep_angle_rad: f64,
    },
    Polyline {
        points: Vec<Point>,
    },
    Polygon {
        points: Vec<Point>,
    },
    Path {
        path: BezPath,
    },
}

impl CanvasShape {
    pub fn to_bezpath(&self) -> BezPath {
        match self {
            CanvasShape::Line { p1, p2 } => KurboLine::new(*p1, *p2).to_path(0.1),
            CanvasShape::Rect { x, y, w, h } => {
                KurboRect::new(*x, *y, x + w, y + h).to_path(0.1)
            }
            CanvasShape::RoundedRect { x, y, w, h, radii } => {
                let rect = KurboRect::new(*x, *y, x + w, y + h);
                KurboRoundedRect::from_rect(rect, (radii.0, radii.1, radii.2, radii.3)).to_path(0.1)
            }
            CanvasShape::Circle { cx, cy, r } => {
                KurboCircle::new(Point::new(*cx, *cy), *r).to_path(0.1)
            }
            CanvasShape::Ellipse { cx, cy, rx, ry } => {
                KurboEllipse::new(Point::new(*cx, *cy), (*rx, *ry), 0.0).to_path(0.1)
            }
            CanvasShape::Arc {
                cx,
                cy,
                r,
                start_angle_rad,
                sweep_angle_rad,
            } => KurboArc {
                center: Point::new(*cx, *cy),
                radii: Vec2::new(*r, *r),
                start_angle: *start_angle_rad,
                sweep_angle: *sweep_angle_rad,
                x_rotation: 0.0,
            }
            .to_path(0.1),
            CanvasShape::Polyline { points } => {
                let mut p = BezPath::new();
                if let Some(first) = points.first() {
                    p.move_to(*first);
                    for pt in &points[1..] {
                        p.line_to(*pt);
                    }
                }
                p
            }
            CanvasShape::Polygon { points } => {
                let mut p = BezPath::new();
                if let Some(first) = points.first() {
                    p.move_to(*first);
                    for pt in &points[1..] {
                        p.line_to(*pt);
                    }
                    p.close_path();
                }
                p
            }
            CanvasShape::Path { path } => path.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CanvasItem {
    pub shape: CanvasShape,
    pub fill: Option<FillStyle>,
    pub stroke: Option<StrokeStyle>,
    pub draw_range: (f64, f64),
}

#[derive(Clone, Debug)]
pub struct CanvasPath {
    pub path: BezPath,
}

impl Default for CanvasPath {
    fn default() -> Self {
        Self::new()
    }
}

impl CanvasPath {
    pub fn new() -> Self {
        Self {
            path: BezPath::new(),
        }
    }

    pub fn move_to(mut self, x: f64, y: f64) -> Self {
        self.path.move_to((x, y));
        self
    }

    pub fn line_to(mut self, x: f64, y: f64) -> Self {
        self.path.line_to((x, y));
        self
    }

    pub fn quad_to(mut self, cx: f64, cy: f64, x: f64, y: f64) -> Self {
        self.path.quad_to((cx, cy), (x, y));
        self
    }

    pub fn cubic_to(mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64) -> Self {
        self.path.curve_to((c1x, c1y), (c2x, c2y), (x, y));
        self
    }

    pub fn close_path(mut self) -> Self {
        self.path.close_path();
        self
    }
}

pub fn trim_bezpath(path: &BezPath, progress: f64) -> BezPath {
    let p = progress.clamp(0.0, 1.0);
    if p >= 1.0 {
        return path.clone();
    }
    if p <= 0.0 {
        return BezPath::new();
    }

    let segments: Vec<vello::kurbo::PathSeg> = path.segments().collect();
    if segments.is_empty() {
        return BezPath::new();
    }

    let seg_lengths: Vec<f64> = segments
        .iter()
        .map(|seg| match seg {
            vello::kurbo::PathSeg::Line(l) => ParamCurveArclen::arclen(l, 0.1),
            vello::kurbo::PathSeg::Quad(q) => ParamCurveArclen::arclen(q, 0.1),
            vello::kurbo::PathSeg::Cubic(c) => ParamCurveArclen::arclen(c, 0.1),
        })
        .collect();
    let total_length: f64 = seg_lengths.iter().sum();
    if total_length == 0.0 {
        return BezPath::new();
    }

    let target_length = total_length * p;
    let mut current_len = 0.0;
    let mut trimmed = BezPath::new();

    for (seg, len) in segments.iter().zip(seg_lengths.iter()) {
        if current_len >= target_length {
            break;
        }

        if current_len + len <= target_length {
            match seg {
                vello::kurbo::PathSeg::Line(l) => {
                    if trimmed.is_empty() {
                        trimmed.move_to(l.p0);
                    }
                    trimmed.line_to(l.p1);
                }
                vello::kurbo::PathSeg::Quad(q) => {
                    if trimmed.is_empty() {
                        trimmed.move_to(q.p0);
                    }
                    trimmed.quad_to(q.p1, q.p2);
                }
                vello::kurbo::PathSeg::Cubic(c) => {
                    if trimmed.is_empty() {
                        trimmed.move_to(c.p0);
                    }
                    trimmed.curve_to(c.p1, c.p2, c.p3);
                }
            }
            current_len += len;
        } else {
            let remaining = target_length - current_len;
            let t = if *len > 0.0 {
                (remaining / len).clamp(0.0, 1.0)
            } else {
                1.0
            };
            let sub_seg = seg.subsegment(0.0..t);

            match sub_seg {
                vello::kurbo::PathSeg::Line(l) => {
                    if trimmed.is_empty() {
                        trimmed.move_to(l.p0);
                    }
                    trimmed.line_to(l.p1);
                }
                vello::kurbo::PathSeg::Quad(q) => {
                    if trimmed.is_empty() {
                        trimmed.move_to(q.p0);
                    }
                    trimmed.quad_to(q.p1, q.p2);
                }
                vello::kurbo::PathSeg::Cubic(c) => {
                    if trimmed.is_empty() {
                        trimmed.move_to(c.p0);
                    }
                    trimmed.curve_to(c.p1, c.p2, c.p3);
                }
            }
            break;
        }
    }

    trimmed
}

#[derive(Clone)]
pub struct Canvas {
    pub size: Size,
    pub padding: Padding,
    pub progress: f64,
    pub items: Vec<CanvasItem>,
    id: Option<NodeId>,
}

impl Component for Canvas {
    fn render(&self, _: &TaffyTree) -> Result<Scene, RenderError> {
        let mut scene = Scene::new();
        let p = self.progress.clamp(0.0, 1.0);

        for item in &self.items {
            let (start, end) = item.draw_range;
            if p < start {
                continue;
            }

            let item_progress = if end > start {
                ((p - start) / (end - start)).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let bezpath = item.shape.to_bezpath();

            // Render Stroke (trimmed by item_progress)
            if let Some(stroke_style) = &item.stroke {
                let trimmed_path = trim_bezpath(&bezpath, item_progress);
                if !trimmed_path.is_empty() {
                    let mut kurbo_stroke = KurboStroke::new(stroke_style.width)
                        .with_caps(stroke_style.cap)
                        .with_join(stroke_style.join);

                    if !stroke_style.dashes.is_empty() {
                        kurbo_stroke = kurbo_stroke.with_dashes(0.0, stroke_style.dashes.clone());
                    }

                    scene.stroke(
                        &kurbo_stroke,
                        Affine::IDENTITY,
                        stroke_style.color,
                        None,
                        &trimmed_path,
                    );
                }
            }

            // Render Fill (fades in smoothly with item_progress)
            if let Some(fill_style) = &item.fill {
                let rgba = fill_style.color.to_rgba8();
                if rgba.a > 0 {
                    let alpha_factor = item_progress;
                    let fill_color = fill_style
                        .color
                        .with_alpha(((rgba.a as f64 / 255.0) * alpha_factor) as f32);

                    scene.fill(
                        fill_style.fill_rule,
                        Affine::IDENTITY,
                        fill_color,
                        None,
                        &bezpath,
                    );
                }
            }
        }

        Ok(scene)
    }

    fn size(&self) -> Result<Size, RenderError> {
        Ok(self.size)
    }

    fn register_layout(
        &mut self,
        tree: &mut TaffyTree,
    ) -> Result<NodeId, RenderError> {
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

impl Canvas {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            padding: Padding::new(0.0),
            progress: 1.0,
            items: Vec::new(),
            id: None,
        }
    }

    pub fn from_size(width: f64, height: f64) -> Self {
        Self::new(Size::new(width, height))
    }

    pub fn progress(&self) -> f64 {
        self.progress
    }

    pub fn set_progress(&mut self, p: f64) {
        self.progress = p.clamp(0.0, 1.0);
    }

    pub fn update_progress(&mut self, p: f64) {
        self.progress = p.clamp(0.0, 1.0);
    }

    pub fn with_progress(mut self, p: f64) -> Self {
        self.progress = p.clamp(0.0, 1.0);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn add_item(
        &mut self,
        shape: CanvasShape,
        fill: Option<FillStyle>,
        stroke: Option<StrokeStyle>,
    ) -> &mut Self {
        self.items.push(CanvasItem {
            shape,
            fill,
            stroke,
            draw_range: (0.0, 1.0),
        });
        self
    }

    pub fn add_item_range(
        &mut self,
        shape: CanvasShape,
        fill: Option<FillStyle>,
        stroke: Option<StrokeStyle>,
        start_p: f64,
        end_p: f64,
    ) -> &mut Self {
        self.items.push(CanvasItem {
            shape,
            fill,
            stroke,
            draw_range: (start_p.clamp(0.0, 1.0), end_p.clamp(0.0, 1.0)),
        });
        self
    }

    pub fn add_line(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        self.add_item(
            CanvasShape::Line {
                p1: Point::new(x1, y1),
                p2: Point::new(x2, y2),
            },
            None,
            stroke,
        )
    }

    pub fn add_rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fill_color: Option<PenikoColor>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let fill = fill_color.map(|c| FillStyle {
            color: c,
            fill_rule: Fill::NonZero,
        });
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        self.add_item(CanvasShape::Rect { x, y, w, h }, fill, stroke)
    }

    pub fn add_rounded_rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rx: f64,
        ry: f64,
        fill_color: Option<PenikoColor>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let fill = fill_color.map(|c| FillStyle {
            color: c,
            fill_rule: Fill::NonZero,
        });
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        self.add_item(
            CanvasShape::RoundedRect {
                x,
                y,
                w,
                h,
                radii: (rx, ry, rx, ry),
            },
            fill,
            stroke,
        )
    }

    pub fn add_circle(
        &mut self,
        cx: f64,
        cy: f64,
        r: f64,
        fill_color: Option<PenikoColor>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let fill = fill_color.map(|c| FillStyle {
            color: c,
            fill_rule: Fill::NonZero,
        });
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        self.add_item(CanvasShape::Circle { cx, cy, r }, fill, stroke)
    }

    pub fn add_ellipse(
        &mut self,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        fill_color: Option<PenikoColor>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let fill = fill_color.map(|c| FillStyle {
            color: c,
            fill_rule: Fill::NonZero,
        });
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        self.add_item(CanvasShape::Ellipse { cx, cy, rx, ry }, fill, stroke)
    }

    pub fn add_arc(
        &mut self,
        cx: f64,
        cy: f64,
        r: f64,
        start_deg: f64,
        sweep_deg: f64,
        fill_color: Option<PenikoColor>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let fill = fill_color.map(|c| FillStyle {
            color: c,
            fill_rule: Fill::NonZero,
        });
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        self.add_item(
            CanvasShape::Arc {
                cx,
                cy,
                r,
                start_angle_rad: start_deg.to_radians(),
                sweep_angle_rad: sweep_deg.to_radians(),
            },
            fill,
            stroke,
        )
    }

    pub fn add_polyline(
        &mut self,
        points: Vec<(f64, f64)>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        let pts = points.into_iter().map(|(x, y)| Point::new(x, y)).collect();
        self.add_item(CanvasShape::Polyline { points: pts }, None, stroke)
    }

    pub fn add_polygon(
        &mut self,
        points: Vec<(f64, f64)>,
        fill_color: Option<PenikoColor>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let fill = fill_color.map(|c| FillStyle {
            color: c,
            fill_rule: Fill::NonZero,
        });
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        let pts = points.into_iter().map(|(x, y)| Point::new(x, y)).collect();
        self.add_item(CanvasShape::Polygon { points: pts }, fill, stroke)
    }

    pub fn add_path(
        &mut self,
        path: BezPath,
        fill_color: Option<PenikoColor>,
        stroke_color: Option<PenikoColor>,
        stroke_width: f64,
    ) -> &mut Self {
        let fill = fill_color.map(|c| FillStyle {
            color: c,
            fill_rule: Fill::NonZero,
        });
        let stroke = stroke_color.map(|c| StrokeStyle {
            color: c,
            width: stroke_width,
            ..Default::default()
        });
        self.add_item(CanvasShape::Path { path }, fill, stroke)
    }
}

pub fn parse_color_val(val: mlua::Value) -> mlua::Result<Option<PenikoColor>> {
    match val {
        mlua::Value::Nil => Ok(None),
        mlua::Value::String(s) => {
            let str_val = s.to_str()?;
            let trimmed = str_val.trim();
            if trimmed.is_empty()
                || trimmed.eq_ignore_ascii_case("none")
                || trimmed.eq_ignore_ascii_case("transparent")
            {
                Ok(None)
            } else if trimmed.starts_with('#') {
                parse_hex_color(trimmed)
            } else if trimmed.len() == 6 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
                parse_hex_color(&format!("#{}", trimmed))
            } else {
                Ok(None)
            }
        }
        mlua::Value::Table(t) => {
            let r: u8 = t.get(1).unwrap_or(0);
            let g: u8 = t.get(2).unwrap_or(0);
            let b: u8 = t.get(3).unwrap_or(0);
            let a: u8 = t.get(4).unwrap_or(255);
            Ok(Some(PenikoColor::from_rgba8(r, g, b, a)))
        }
        _ => Ok(None),
    }
}

fn parse_hex_color(hex: &str) -> mlua::Result<Option<PenikoColor>> {
    let hex = hex.trim_start_matches('#');
    let (r, g, b, a) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0);
            (r, g, b, 255)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            (r, g, b, a)
        }
        _ => return Ok(None),
    };
    Ok(Some(PenikoColor::from_rgba8(r, g, b, a)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_bezpath_standalone() {
        let mut canvas = Canvas::from_size(400.0, 300.0);
        canvas.add_line(10.0, 10.0, 100.0, 100.0, Some(PenikoColor::WHITE), 2.0);
        canvas.add_circle(200.0, 150.0, 40.0, None, Some(PenikoColor::WHITE), 2.0);

        let trimmed_canvas = canvas.with_progress(0.5);
        let scene = trimmed_canvas.render(&TaffyTree::new()).unwrap();
        assert_eq!(trimmed_canvas.progress(), 0.5);
    }
}
