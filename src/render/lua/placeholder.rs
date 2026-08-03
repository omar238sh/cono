use mlua::{UserData, Value};

use crate::render::{
    components::placeholder::{ElementPointer, PlaceHolder},
    error::RenderError,
    frame::Component,
};

impl<T: 'static> mlua::FromLua for ElementPointer<T> {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "ElementPointer".to_string(),
            message: Some("expected ElementPointer UserData".to_string()),
        })
    }
}

impl<T: 'static + UserData> mlua::UserData for ElementPointer<T> {
    // fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
    //     fields.add_field_method_get("id", |_, this| Ok(this.id.0));
    // }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("drop_component", |_, this, ()| {
            this.drop_component();
            Ok(())
        });

        methods.add_method("with", |lua, this, func: mlua::Function| {
            let a = this
                .with(|a| {
                    lua.scope(|scope| {
                        let a_ref = scope.create_userdata_ref(a)?;
                        func.call::<Value>(a_ref)?;
                        Ok(())
                    })
                    .map_err(|e| RenderError::CustomError(e.to_string()))?;
                    Ok::<(), RenderError>(())
                })
                .map_err(|e| mlua::Error::external(format!("Error : {e}")))?;
            a.map_err(|e| mlua::Error::external(format!("Error : {e}")))?;
            Ok(())
        });

        methods.add_method_mut("with_mut", |lua, this, func: mlua::Function| {
            let a = this
                .with_mut(|a| {
                    lua.scope(|scope| {
                        let a_ref = scope.create_userdata_ref_mut(a)?;
                        func.call::<Value>(a_ref)?;
                        Ok(())
                    })
                    .map_err(|e| RenderError::CustomError(e.to_string()))?;
                    Ok::<(), RenderError>(())
                })
                .map_err(|e| mlua::Error::external(format!("Error : {e}")))?;
            a.map_err(|e| mlua::Error::external(format!("Error : {e}")))?;
            Ok(())
        });
    }
}

impl<C: Component + Clone + 'static + UserData> mlua::UserData for PlaceHolder<C> {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("pointer", |_, this, ()| Ok(this.pointer));
    }
}
