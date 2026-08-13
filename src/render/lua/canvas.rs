use crate::render::{
    components::canvas::{Canvas, CanvasPath, parse_color_val},
    components::style::Padding,
    models::Size,
};

impl mlua::FromLua for Canvas {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Canvas".to_string(),
            message: Some("expected Canvas UserData".to_string()),
        })
    }
}

impl mlua::FromLua for CanvasPath {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "CanvasPath".to_string(),
            message: Some("expected CanvasPath UserData".to_string()),
        })
    }
}

impl mlua::UserData for CanvasPath {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, ()| Ok(CanvasPath::new()));

        methods.add_method("move_to", |_, this, (x, y): (f64, f64)| {
            Ok(this.clone().move_to(x, y))
        });

        methods.add_method("line_to", |_, this, (x, y): (f64, f64)| {
            Ok(this.clone().line_to(x, y))
        });

        methods.add_method("quad_to", |_, this, (cx, cy, x, y): (f64, f64, f64, f64)| {
            Ok(this.clone().quad_to(cx, cy, x, y))
        });

        methods.add_method(
            "cubic_to",
            |_, this, (c1x, c1y, c2x, c2y, x, y): (f64, f64, f64, f64, f64, f64)| {
                Ok(this.clone().cubic_to(c1x, c1y, c2x, c2y, x, y))
            },
        );

        methods.add_method("close_path", |_, this, ()| Ok(this.clone().close_path()));
    }
}

impl mlua::UserData for Canvas {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, size: Size| Ok(Canvas::new(size)));
        methods.add_function("from_size", |_, (w, h): (f64, f64)| {
            Ok(Canvas::from_size(w, h))
        });

        methods.add_method("size", |_, this, ()| Ok(this.size));

        methods.add_method("progress", |_, this, ()| Ok(this.progress()));
        methods.add_method_mut("set_progress", |_, this, p: f64| {
            this.set_progress(p);
            Ok(())
        });
        methods.add_method_mut("update_progress", |_, this, p: f64| {
            this.update_progress(p);
            Ok(())
        });
        methods.add_method("with_progress", |_, this, p: f64| {
            Ok(this.clone().with_progress(p))
        });

        methods.add_method("padding", |_, this, pad: Padding| {
            Ok(this.clone().padding(pad))
        });
        methods.add_method_mut("set_padding", |_, this, pad: Padding| {
            this.set_padding(pad);
            Ok(())
        });

        methods.add_method_mut("clear", |_, this, ()| {
            this.clear();
            Ok(())
        });

        // Shape methods
        methods.add_method_mut(
            "add_line",
            |_,
             this,
             (x1, y1, x2, y2, stroke_color_val, stroke_width): (
                f64,
                f64,
                f64,
                f64,
                mlua::Value,
                Option<f64>,
            )| {
                let stroke_color = parse_color_val(stroke_color_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_line(x1, y1, x2, y2, stroke_color, width);
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_rect",
            |_,
             this,
             (x, y, w, h, fill_val, stroke_val, stroke_width): (
                f64,
                f64,
                f64,
                f64,
                mlua::Value,
                mlua::Value,
                Option<f64>,
            )| {
                let fill_color = parse_color_val(fill_val)?;
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_rect(x, y, w, h, fill_color, stroke_color, width);
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_rounded_rect",
            |_,
             this,
             (x, y, w, h, rx, ry, fill_val, stroke_val, stroke_width): (
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                mlua::Value,
                mlua::Value,
                Option<f64>,
            )| {
                let fill_color = parse_color_val(fill_val)?;
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_rounded_rect(x, y, w, h, rx, ry, fill_color, stroke_color, width);
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_circle",
            |_,
             this,
             (cx, cy, r, fill_val, stroke_val, stroke_width): (
                f64,
                f64,
                f64,
                mlua::Value,
                mlua::Value,
                Option<f64>,
            )| {
                let fill_color = parse_color_val(fill_val)?;
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_circle(cx, cy, r, fill_color, stroke_color, width);
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_ellipse",
            |_,
             this,
             (cx, cy, rx, ry, fill_val, stroke_val, stroke_width): (
                f64,
                f64,
                f64,
                f64,
                mlua::Value,
                mlua::Value,
                Option<f64>,
            )| {
                let fill_color = parse_color_val(fill_val)?;
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_ellipse(cx, cy, rx, ry, fill_color, stroke_color, width);
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_arc",
            |_,
             this,
             (cx, cy, r, start_deg, sweep_deg, fill_val, stroke_val, stroke_width): (
                f64,
                f64,
                f64,
                f64,
                f64,
                mlua::Value,
                mlua::Value,
                Option<f64>,
            )| {
                let fill_color = parse_color_val(fill_val)?;
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_arc(
                    cx,
                    cy,
                    r,
                    start_deg,
                    sweep_deg,
                    fill_color,
                    stroke_color,
                    width,
                );
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_polyline",
            |_,
             this,
             (pts_table, stroke_val, stroke_width): (mlua::Table, mlua::Value, Option<f64>)| {
                let mut points = Vec::new();
                for pair in pts_table.sequence_values::<mlua::Table>() {
                    if let Ok(p) = pair {
                        let x: f64 = p.get(1).unwrap_or(0.0);
                        let y: f64 = p.get(2).unwrap_or(0.0);
                        points.push((x, y));
                    }
                }
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_polyline(points, stroke_color, width);
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_polygon",
            |_,
             this,
             (pts_table, fill_val, stroke_val, stroke_width): (
                mlua::Table,
                mlua::Value,
                mlua::Value,
                Option<f64>,
            )| {
                let mut points = Vec::new();
                for pair in pts_table.sequence_values::<mlua::Table>() {
                    if let Ok(p) = pair {
                        let x: f64 = p.get(1).unwrap_or(0.0);
                        let y: f64 = p.get(2).unwrap_or(0.0);
                        points.push((x, y));
                    }
                }
                let fill_color = parse_color_val(fill_val)?;
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_polygon(points, fill_color, stroke_color, width);
                Ok(())
            },
        );

        methods.add_method_mut(
            "add_path",
            |_,
             this,
             (path_obj, fill_val, stroke_val, stroke_width): (
                CanvasPath,
                mlua::Value,
                mlua::Value,
                Option<f64>,
            )| {
                let fill_color = parse_color_val(fill_val)?;
                let stroke_color = parse_color_val(stroke_val)?;
                let width = stroke_width.unwrap_or(2.0);
                this.add_path(path_obj.path, fill_color, stroke_color, width);
                Ok(())
            },
        );
    }
}
