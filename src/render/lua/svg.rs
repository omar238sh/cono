use std::path::Path;

use crate::render::{
    components::{style::Padding, svg::SvgBox},
    models::Size,
};

impl mlua::FromLua for SvgBox {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "SvgBox".to_string(),
            message: Some("expected SvgBox UserData".to_string()),
        })
    }
}

impl mlua::UserData for SvgBox {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.size));
        fields.add_field_method_get("padding", |_, this| Ok(this.padding));
        fields.add_field_method_set("padding", |_, this, pad: Padding| {
            this.padding = pad;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, content: String| {
            SvgBox::new(content).map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_function("new_static", |_, content: String| {
            let mut svg =
                SvgBox::new(content).map_err(|e| mlua::Error::external(format!("{:?}", e)))?;

            svg.set_progress(1.0);
            svg.pin();

            Ok(svg)
        });

        methods.add_function("from_path", |lua, path_str: String| {
            let raw = Path::new(&path_str);

            let resolved: std::path::PathBuf = if raw.is_absolute() {
                raw.to_path_buf()
            } else {
                let mut dir = None;
                for level in 0..16 {
                    let found = lua.inspect_stack(level, |dbg| {
                        dbg.source().source.and_then(|src| {
                            src.strip_prefix('@')
                                .and_then(|file| Path::new(file).parent())
                                .map(|p| p.to_path_buf())
                        })
                    });
                    match found {
                        Some(Some(d)) => {
                            dir = Some(d);
                            break;
                        }
                        Some(None) => continue, // frame exists but not a file chunk (C fn, string chunk, etc.)
                        None => break,          // no more frames
                    }
                }
                dir.map(|d| d.join(raw))
                    .unwrap_or_else(|| raw.to_path_buf())
            };

            SvgBox::from_path(&resolved).map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });
        methods.add_method("padding", |_, this, pad: Padding| {
            Ok(this.clone().padding(pad))
        });

        methods.add_method_mut("set_padding", |_, this, pad: Padding| {
            this.set_padding(pad);
            Ok(())
        });

        methods.add_method("with_size", |_, this, size: Size| {
            Ok(this.clone().with_size(size))
        });
        methods.add_method("scale", |_, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
        methods.add_method_mut("set_scale", |_, this, factor: f64| {
            this.set_scale(factor);
            Ok(())
        });
        methods.add_method_mut("set_progress", |_, this, progress: f64| {
            this.set_progress(progress);
            Ok(())
        });

        methods.add_method_mut("pin", |_, this, ()| {
            this.pin();
            Ok(())
        });

        methods.add_method_mut("unpin", |_, this, ()| {
            this.unpin();
            Ok(())
        });

        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("SvgBox({}x{})", this.size.width, this.size.height))
        });
    }
}
