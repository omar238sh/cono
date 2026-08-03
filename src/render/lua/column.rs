use mlua::UserData;

use crate::{
    down_cast,
    render::{
        components::{
            column::Column,
            placeholder::PlaceHolder,
            style::{Alignment, Arrangement, Padding},
        },
        models::Size,
    },
};

impl mlua::FromLua for Column {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Column".to_string(),
            message: Some("expected Column UserData".to_string()),
        })
    }
}

impl UserData for Column {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.style.size()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, size: Size| Ok(Column::new(size)));
        methods.add_method_mut("push", |_, this, val: mlua::AnyUserData| {
            down_cast!(val, |c| {
                this.push(c);
            });
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
