pub mod animengin;
pub mod model;

pub use model::{Easing, Model};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FpsTarget {
    F30,
    F45,
    F60,
    F120,
    Custom(u32),
}

impl mlua::FromLua for FpsTarget {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "FpsTarget".to_string(),
            message: Some("expected FpsTarget UserData".to_string()),
        })
    }
}

impl FpsTarget {
    pub fn value(&self) -> u32 {
        match self {
            FpsTarget::F30 => 30,
            FpsTarget::F45 => 45,
            FpsTarget::F60 => 60,
            FpsTarget::F120 => 120,
            FpsTarget::Custom(v) => v.clone(),
        }
    }
}

impl mlua::UserData for FpsTarget {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| Ok(this.value()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("value", |_, this, ()| Ok(this.value()));
        methods.add_function("f30", |_, ()| Ok(FpsTarget::F30));
        methods.add_function("f45", |_, ()| Ok(FpsTarget::F45));
        methods.add_function("f60", |_, ()| Ok(FpsTarget::F60));
        methods.add_function("f120", |_, ()| Ok(FpsTarget::F120));
        methods.add_function("custom", |_, fps: u32| Ok(FpsTarget::Custom(fps)));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("FpsTarget({})", this.value()))
        });
    }
}
