#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl mlua::FromLua for Size {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Size".to_string(),
            message: Some("expected Size UserData".to_string()),
        })
    }
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn x_landscape() -> Self {
        Self::new(1200.0, 675.0)
    }

    pub fn tiktok() -> Self {
        Self::new(1080.0, 1920.0)
    }

    pub fn instagram_reels() -> Self {
        Self::new(1080.0, 1920.0)
    }

    pub fn instagram_portrait() -> Self {
        Self::new(1080.0, 1350.0)
    }

    pub fn instagram_square() -> Self {
        Self::new(1080.0, 1080.0)
    }

    pub fn youtube_shorts() -> Self {
        Self::new(1080.0, 1920.0)
    }

    pub fn youtube_video() -> Self {
        Self::new(1920.0, 1080.0)
    }
}

use mlua::{Lua, Result as LuaResult, Table, UserData, UserDataFields, UserDataMethods};

impl UserData for Size {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("width", |_, s| Ok(s.width));
        fields.add_field_method_set("width", |_, s, v: f64| {
            s.width = v;
            Ok(())
        });

        fields.add_field_method_get("height", |_, s| Ok(s.height));
        fields.add_field_method_set("height", |_, s, v: f64| {
            s.height = v;
            Ok(())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, (width, height): (f64, f64)| {
            Ok(Size::new(width, height))
        });
        methods.add_function("x_landscape", |_, ()| Ok(Size::x_landscape()));
        methods.add_function("tiktok", |_, ()| Ok(Size::tiktok()));
        methods.add_function("instagram_reels", |_, ()| Ok(Size::instagram_reels()));
        methods.add_function("instagram_portrait", |_, ()| Ok(Size::instagram_portrait()));
        methods.add_function("instagram_square", |_, ()| Ok(Size::instagram_square()));
        methods.add_function("youtube_shorts", |_, ()| Ok(Size::youtube_shorts()));
        methods.add_function("youtube_video", |_, ()| Ok(Size::youtube_video()));

        methods.add_method("aspect_ratio", |_, s, ()| Ok(s.width / s.height));

        methods.add_meta_method(mlua::MetaMethod::Eq, |_, s, other: Size| Ok(*s == other));
        methods.add_meta_method(mlua::MetaMethod::Mul, |_, s, factor: f64| {
            Ok(Size::new(s.width * factor, s.height * factor))
        });
        methods.add_meta_method(mlua::MetaMethod::Div, |_, s, factor: f64| {
            Ok(Size::new(s.width / factor, s.height / factor))
        });
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, s, ()| {
            Ok(format!("Size({}x{})", s.width, s.height))
        });
    }
}

pub fn register_size(lua: &Lua) -> LuaResult<()> {
    let size_table: Table = lua.create_table()?;

    size_table.set(
        "new",
        lua.create_function(|_, (w, h): (f64, f64)| Ok(Size::new(w, h)))?,
    )?;
    size_table.set(
        "x_landscape",
        lua.create_function(|_, ()| Ok(Size::x_landscape()))?,
    )?;
    size_table.set("tiktok", lua.create_function(|_, ()| Ok(Size::tiktok()))?)?;
    size_table.set(
        "instagram_reels",
        lua.create_function(|_, ()| Ok(Size::instagram_reels()))?,
    )?;
    size_table.set(
        "instagram_portrait",
        lua.create_function(|_, ()| Ok(Size::instagram_portrait()))?,
    )?;
    size_table.set(
        "instagram_square",
        lua.create_function(|_, ()| Ok(Size::instagram_square()))?,
    )?;
    size_table.set(
        "youtube_shorts",
        lua.create_function(|_, ()| Ok(Size::youtube_shorts()))?,
    )?;
    size_table.set(
        "youtube_video",
        lua.create_function(|_, ()| Ok(Size::youtube_video()))?,
    )?;

    lua.globals().set("Size", size_table)?;
    Ok(())
}
