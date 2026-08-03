use crate::render::components::text::{FontStyle, Text, Weight};

impl mlua::FromLua for Text {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Text".to_string(),
            message: Some("expected Text UserData".to_string()),
        })
    }
}
impl mlua::FromLua for Weight {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Weight".to_string(),
            message: Some("expected Weight UserData".to_string()),
        })
    }
}

impl mlua::FromLua for FontStyle {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "FontStyle".to_string(),
            message: Some("expected FontStyle UserData".to_string()),
        })
    }
}
impl mlua::UserData for Weight {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("thin", |_, ()| Ok(Weight::Thin));
        methods.add_function("extra_light", |_, ()| Ok(Weight::ExtraLight));
        methods.add_function("light", |_, ()| Ok(Weight::Light));
        methods.add_function("regular", |_, ()| Ok(Weight::Reqular));
        methods.add_function("medium", |_, ()| Ok(Weight::Medium));
        methods.add_function("semi_bold", |_, ()| Ok(Weight::SemiBold));
        methods.add_function("bold", |_, ()| Ok(Weight::Bold));
        methods.add_function("extra_bold", |_, ()| Ok(Weight::ExtraBold));
        methods.add_function("black", |_, ()| Ok(Weight::Black));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("Weight::{}", this.as_typst()))
        });
    }
}

impl mlua::UserData for FontStyle {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("regular", |_, ()| Ok(FontStyle::Regular));
        methods.add_function("italic", |_, ()| Ok(FontStyle::Italic));
        methods.add_function("oblique", |_, ()| Ok(FontStyle::Oblique));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("FontStyle::{}", this.as_typst()))
        });
    }
}

impl mlua::UserData for Text {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("content", |_, this| Ok(this.content.clone()));
        fields.add_field_method_set("content", |_, this, val: String| {
            this.content = val;
            this.cache = None;
            Ok(())
        });
        fields.add_field_method_get("color", |_, this| Ok(this.color.clone()));
        fields.add_field_method_set("color", |_, this, val: Option<String>| {
            this.color = val;
            this.cache = None;
            Ok(())
        });
        fields.add_field_method_get("font", |_, this| Ok(this.font.clone()));
        fields.add_field_method_set("font", |_, this, val: Option<String>| {
            this.font = val;
            this.cache = None;
            Ok(())
        });
        fields.add_field_method_get("size", |_, this| Ok(this.size));
        fields.add_field_method_set("size", |_, this, val: Option<u32>| {
            this.size = val;
            this.cache = None;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, content: String| Ok(Text::new(content)));
        methods.add_method("color", |_, this, c: String| Ok(this.clone().color(c)));
        methods.add_method_mut("set_color", |_, this, c: String| {
            this.set_color(c);
            Ok(())
        });
        methods.add_method("font", |_, this, f: String| Ok(this.clone().font(f)));
        methods.add_method_mut("set_font", |_, this, f: String| {
            this.set_font(f);
            Ok(())
        });
        methods.add_method("size", |_, this, s: u32| Ok(this.clone().size(s)));
        methods.add_method_mut("set_size", |_, this, s: u32| {
            this.set_size(s);
            Ok(())
        });
        methods.add_method("weight", |_, this, w: Weight| Ok(this.clone().weight(w)));
        methods.add_method_mut("set_weight", |_, this, w: Weight| {
            this.set_weight(w);
            Ok(())
        });
        methods.add_method("italic", |_, this, ()| Ok(this.clone().italic()));
        methods.add_method_mut("set_italic", |_, this, ()| {
            this.set_italic();
            Ok(())
        });
        methods.add_method("to_typst", |_, this, ()| Ok(this.to_typst()));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("Text(\"{}\")", this.content))
        });
    }
}
