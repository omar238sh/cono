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
        methods.add_function("from_path", |_, path_str: String| {
            SvgBox::from_path(Path::new(&path_str))
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
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
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("SvgBox({}x{})", this.size.width, this.size.height))
        });
    }
}
