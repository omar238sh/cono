use std::{fs, path::Path};

use mlua_extras::mlua::Lua;

use crate::render::{
    animation::{FpsTarget, Model},
    components::{
        freebox::{Anchor, Coordinate, Location},
        text::Text,
    },
    models::Size,
};

impl mlua_extras::mlua::UserData for Size {
    fn add_methods<M: mlua_extras::mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("width", |_lua, this, ()| Ok(this.width));
        methods.add_method("height", |_lua, this, ()| Ok(this.height));
        methods.add_method_mut("set_width", |_lua, this, width: f64| {
            this.width = width;
            Ok(())
        });
        methods.add_method_mut("set_height", |_lua, this, height: f64| {
            this.height = height;
            Ok(())
        });
    }
}

impl mlua_extras::mlua::UserData for Text {
    fn add_methods<M: mlua_extras::mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_content", |_lua, this, content: String| {
            this.content = content;
            Ok(())
        });
        methods.add_method_mut("set_color", |_lua, this, color: String| {
            this.set_color(color);
            Ok(())
        });
        methods.add_method_mut("set_font", |_lua, this, font: String| {
            this.set_font(font);
            Ok(())
        });
        methods.add_method_mut("set_size", |_lua, this, size: u32| {
            this.set_size(size);
            Ok(())
        });
        methods.add_method_mut("set_italic", |_lua, this, ()| {
            this.set_italic();
            Ok(())
        });
    }
}

impl mlua_extras::mlua::UserData for Coordinate {
    fn add_methods<M: mlua_extras::mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("to_num", |_lua, this, max_length: f64| {
            Ok(this.to_num(max_length))
        });
        methods.add_method("scale", |_lua, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
    }
}

impl mlua_extras::mlua::UserData for Location {
    fn add_methods<M: mlua_extras::mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_progress", |_lua, this, progress: f64| {
            this.set_progress(progress)
                .map_err(|e| mlua_extras::mlua::Error::external(format!("{:?}", e)))
        });
    }
}

fn fps_from(value: Option<u32>) -> FpsTarget {
    match value {
        Some(30) => FpsTarget::F30,
        Some(45) => FpsTarget::F45,
        Some(60) => FpsTarget::F60,
        Some(120) => FpsTarget::F120,
        Some(v) => FpsTarget::Custom(v),
        None => FpsTarget::F30,
    }
}

fn anchor_from(value: String) -> Anchor {
    match value.to_lowercase().as_str() {
        "center" => Anchor::Center,
        "topright" | "top_right" => Anchor::TopRight,
        "bottomleft" | "bottom_left" => Anchor::BottomLeft,
        "bottomright" | "bottom_right" => Anchor::BottomRight,
        _ => Anchor::TopLeft,
    }
}

pub fn new_lua() -> mlua_extras::mlua::Result<Lua> {
    let lua = Lua::new();
    let globals = lua.globals();

    let cono = lua.create_table()?;

    let size_table = lua.create_table()?;
    size_table.set("new", lua.create_function(|_, (w, h): (f64, f64)| Ok(Size::new(w, h)))?)?;
    size_table.set(
        "youtube_shorts",
        lua.create_function(|_, ()| Ok(Size::youtube_shorts()))?,
    )?;
    size_table.set("youtube_video", lua.create_function(|_, ()| Ok(Size::youtube_video()))?)?;
    size_table.set("tiktok", lua.create_function(|_, ()| Ok(Size::tiktok()))?)?;
    size_table.set("x_landscape", lua.create_function(|_, ()| Ok(Size::x_landscape()))?)?;
    cono.set("Size", size_table)?;

    let text_table = lua.create_table()?;
    text_table.set(
        "new",
        lua.create_function(|_, content: String| Ok(Text::new(content)))?,
    )?;
    cono.set("Text", text_table)?;

    let coordinate_table = lua.create_table()?;
    coordinate_table.set(
        "absolute",
        lua.create_function(|_, value: f64| Ok(Coordinate::absolute(value)))?,
    )?;
    coordinate_table.set(
        "percentage",
        lua.create_function(|_, value: f64| Ok(Coordinate::percentage(value)))?,
    )?;
    coordinate_table.set("zero", lua.create_function(|_, ()| Ok(Coordinate::zero()))?)?;
    cono.set("Coordinate", coordinate_table)?;

    let location_table = lua.create_table()?;
    location_table.set(
        "new",
        lua.create_function(|_, (x, y): (f64, f64)| {
            Ok(Location::new(
                Coordinate::absolute(x),
                Coordinate::absolute(y),
            ))
        })?,
    )?;
    location_table.set(
        "center",
        lua.create_function(|_, anchor: Option<String>| {
            Ok(Location::center(anchor.map(anchor_from).unwrap_or(Anchor::Center)))
        })?,
    )?;
    cono.set("Location", location_table)?;

    let model_table = lua.create_table()?;
    model_table.set(
        "new",
        lua.create_function(|_, (w, h, fps): (f64, f64, Option<u32>)| {
            Ok(Model::new(Size::new(w, h), fps_from(fps)))
        })?,
    )?;
    cono.set("Model", model_table)?;

    globals.set("cono", cono)?;
    Ok(lua)
}

pub fn run_script(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let lua = new_lua()?;
    let source = fs::read_to_string(path)?;
    lua.load(source).set_name(path.to_string_lossy().as_ref()).exec()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::new_lua;

    #[test]
    fn test_lua_model_capture_frames() {
        let lua = new_lua().unwrap();
        let script = r#"
            local model = cono.Model.new(1080, 1920, 30)
            local ptr = model:spawn_text("hello", 10, 20)
            ptr:set_position(100, 120)
            model:capture_frame()
            ptr:set_position(140, 120)
            model:capture_frame()
            return model:frames_count()
        "#;
        let frames: usize = lua.load(script).eval().unwrap();
        assert_eq!(frames, 2);
    }
}
