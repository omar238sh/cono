use mlua::Lua;

use crate::render::{
    animation::{Easing, FpsTarget, Model},
    components::{
        column::Column,
        container::Container,
        freebox::{Anchor, Angle, Coordinate, FreeBox, LocatedComponent, Location, Opacity},
        placeholder::PlaceHolder,
        row::Row,
        style::{Alignment, Arrangement, Padding, Style},
        svg::SvgBox,
        text::{FontStyle, Text, Weight},
    },
    frame::Frame,
    models::{Size, register_size},
    scene::{RenderedFrame, Scene},
    typst::{LuaWorld, pager::Pager},
};

pub fn register_globals(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    register_size(lua)?;

    globals.set("Size", lua.create_proxy::<Size>()?)?;
    globals.set("Text", lua.create_proxy::<Text>()?)?;
    globals.set("Weight", lua.create_proxy::<Weight>()?)?;
    globals.set("FontStyle", lua.create_proxy::<FontStyle>()?)?;
    globals.set("Column", lua.create_proxy::<Column>()?)?;
    globals.set("Row", lua.create_proxy::<Row>()?)?;
    globals.set("Container", lua.create_proxy::<Container>()?)?;
    globals.set("FreeBox", lua.create_proxy::<FreeBox>()?)?;
    globals.set("SvgBox", lua.create_proxy::<SvgBox>()?)?;
    globals.set("LocatedComponent", lua.create_proxy::<LocatedComponent>()?)?;
    globals.set(
        "PlaceHolderLocatedComponent",
        lua.create_proxy::<PlaceHolder<LocatedComponent>>()?,
    )?;
    globals.set("PlaceHolderText", lua.create_proxy::<PlaceHolder<Text>>()?)?;
    globals.set(
        "PlaceHolderColumn",
        lua.create_proxy::<PlaceHolder<Column>>()?,
    )?;
    globals.set("PlaceHolderRow", lua.create_proxy::<PlaceHolder<Row>>()?)?;
    globals.set(
        "PlaceHolderContainer",
        lua.create_proxy::<PlaceHolder<Container>>()?,
    )?;
    globals.set(
        "PlaceHolderFreeBox",
        lua.create_proxy::<PlaceHolder<FreeBox>>()?,
    )?;
    globals.set(
        "PlaceHolderSvgBox",
        lua.create_proxy::<PlaceHolder<SvgBox>>()?,
    )?;
    globals.set("Model", lua.create_proxy::<Model>()?)?;
    globals.set("Easing", lua.create_proxy::<Easing>()?)?;
    globals.set("Coordinate", lua.create_proxy::<Coordinate>()?)?;
    globals.set("Location", lua.create_proxy::<Location>()?)?;
    globals.set("Anchor", lua.create_proxy::<Anchor>()?)?;
    globals.set("Opacity", lua.create_proxy::<Opacity>()?)?;
    globals.set("Angle", lua.create_proxy::<Angle>()?)?;
    globals.set("FpsTarget", lua.create_proxy::<FpsTarget>()?)?;
    globals.set("Arrangement", lua.create_proxy::<Arrangement>()?)?;
    globals.set("Alignment", lua.create_proxy::<Alignment>()?)?;
    globals.set("Padding", lua.create_proxy::<Padding>()?)?;
    globals.set("Style", lua.create_proxy::<Style>()?)?;
    globals.set("LuaWorld", lua.create_proxy::<LuaWorld>()?)?;
    globals.set("Pager", lua.create_proxy::<Pager>()?)?;
    globals.set("Frame", lua.create_proxy::<Frame>()?)?;
    globals.set("Scene", lua.create_proxy::<Scene>()?)?;
    globals.set("RenderedFrame", lua.create_proxy::<RenderedFrame>()?)?;

    Ok(())
}
