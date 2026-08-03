use crate::render::{
    components::style::{Alignment, Arrangement, Padding, Style},
    models::Size,
};

impl mlua::FromLua for Style {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Style".to_string(),
            message: Some("expected Style UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Arrangement {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Arrangement".to_string(),
            message: Some("expected Arrangement UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Alignment {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Alignment".to_string(),
            message: Some("expected Alignment UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Padding {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Padding".to_string(),
            message: Some("expected Padding UserData".to_string()),
        })
    }
}

impl mlua::UserData for Style {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.size()));
        fields.add_field_method_get("arrangement", |_, this| Ok(this.arrangement().clone()));
        fields.add_field_method_set("arrangement", |_, this, arr: Arrangement| {
            *this.arrangement_mut() = arr;
            Ok(())
        });
        fields.add_field_method_get("alignment", |_, this| Ok(this.alignment().clone()));
        fields.add_field_method_set("alignment", |_, this, align: Alignment| {
            *this.alignment_mut() = align;
            Ok(())
        });
        fields.add_field_method_get("padding", |_, this| Ok(this.padding().clone()));
        fields.add_field_method_set("padding", |_, this, pad: Padding| {
            *this.padding_mut() = pad;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, size: Size| Ok(Style::new(size)));
        methods.add_method("size", |_, this, ()| Ok(this.size()));
        methods.add_method("arrangement", |_, this, ()| Ok(this.arrangement().clone()));
        methods.add_method_mut("set_arrangement", |_, this, arr: Arrangement| {
            *this.arrangement_mut() = arr;
            Ok(())
        });
        methods.add_method("alignment", |_, this, ()| Ok(this.alignment().clone()));
        methods.add_method_mut("set_alignment", |_, this, align: Alignment| {
            *this.alignment_mut() = align;
            Ok(())
        });
        methods.add_method("padding", |_, this, ()| Ok(this.padding().clone()));
        methods.add_method_mut("set_padding", |_, this, pad: Padding| {
            *this.padding_mut() = pad;
            Ok(())
        });
    }
}

impl mlua::UserData for Arrangement {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("space_between", |_, ()| Ok(Arrangement::SpaceBetween));
        methods.add_function("spaced_by", |_, pixels: u32| {
            Ok(Arrangement::SpacedBy(pixels))
        });
        methods.add_function("start", |_, ()| Ok(Arrangement::Start));
        methods.add_function("end", |_, ()| Ok(Arrangement::End));
        methods.add_function("space_around", |_, ()| Ok(Arrangement::SpaceAround));
        methods.add_function("space_evenly", |_, ()| Ok(Arrangement::SpaceEvenly));
        methods.add_function("center", |_, ()| Ok(Arrangement::Center));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(match this {
                Arrangement::SpaceBetween => "Arrangement::SpaceBetween".to_string(),
                Arrangement::SpacedBy(p) => format!("Arrangement::SpacedBy({})", p),
                Arrangement::Start => "Arrangement::Start".to_string(),
                Arrangement::End => "Arrangement::End".to_string(),
                Arrangement::SpaceAround => "Arrangement::SpaceAround".to_string(),
                Arrangement::SpaceEvenly => "Arrangement::SpaceEvenly".to_string(),
                Arrangement::Center => "Arrangement::Center".to_string(),
            })
        });
    }
}

impl mlua::UserData for Alignment {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("start", |_, ()| Ok(Alignment::Start));
        methods.add_function("end", |_, ()| Ok(Alignment::End));
        methods.add_function("center", |_, ()| Ok(Alignment::Center));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(match this {
                Alignment::Start => "Alignment::Start".to_string(),
                Alignment::End => "Alignment::End".to_string(),
                Alignment::Center => "Alignment::Center".to_string(),
            })
        });
    }
}

impl mlua::UserData for Padding {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("left", |_, this| Ok(this.left));
        fields.add_field_method_set("left", |_, this, val: f64| {
            this.left = val;
            Ok(())
        });
        fields.add_field_method_get("right", |_, this| Ok(this.right));
        fields.add_field_method_set("right", |_, this, val: f64| {
            this.right = val;
            Ok(())
        });
        fields.add_field_method_get("top", |_, this| Ok(this.top));
        fields.add_field_method_set("top", |_, this, val: f64| {
            this.top = val;
            Ok(())
        });
        fields.add_field_method_get("bottom", |_, this| Ok(this.bottom));
        fields.add_field_method_set("bottom", |_, this, val: f64| {
            this.bottom = val;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, val: f64| Ok(Padding::new(val)));
        methods.add_method_mut("edit", |_, this, val: f64| {
            this.edit(val);
            Ok(())
        });
        methods.add_method("left", |_, this, val: f64| Ok(this.left(val)));
        methods.add_method("right", |_, this, val: f64| Ok(this.right(val)));
        methods.add_method("top", |_, this, val: f64| Ok(this.top(val)));
        methods.add_method("bottom", |_, this, val: f64| Ok(this.bottom(val)));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!(
                "Padding(l:{}, r:{}, t:{}, b:{})",
                this.left, this.right, this.top, this.bottom
            ))
        });
    }
}
