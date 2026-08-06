use vello::peniko::Color;

use crate::{
    down_cast,
    render::{
        components::{container::Container, placeholder::PlaceHolder, style::Padding},
        models::Size,
    },
};

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
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, child: mlua::AnyUserData| {
            let a = down_cast!(child, |c| {
                Container::new(c).map_err(|e| mlua::Error::external(format!("{:?}", e)))
            })
            .unwrap();
            a
        });

        methods.add_function("from_size", |_, size: Size| {
            let a = Container::from_size(size);
            Ok(a)
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
