use crate::render::{animation::Model, lua::lua::register_globals};
use mlua::Lua;
use std::path::Path;

fn create_lua_instance() -> mlua::Result<Lua> {
    let lua = Lua::new();
    register_globals(&lua)?;
    Ok(lua)
}

#[test]
fn test_lua_primitives_and_types() -> mlua::Result<()> {
    let lua = create_lua_instance()?;

    let script = r##"
        local sz = Size.youtube_video()
        assert(sz.width == 1920 and sz.height == 1080, "Size preset failed")

        local custom_sz = Size.new(800, 600)
        assert(custom_sz.width == 800 and custom_sz.height == 600, "Size.new failed")

        local fps = FpsTarget.f60()
        assert(fps.value == 60, "FpsTarget failed")

        local ease = Easing.ease_out()
        assert(ease ~= nil, "Easing failed")

        local coord_abs = Coordinate.absolute(150.0)
        local coord_pct = Coordinate.percentage(0.5)
        local coord_prog = Coordinate.with_progress(coord_abs, coord_pct)

        local loc = Location.new(coord_abs, coord_pct)
        loc.anchor = Anchor.center()
        assert(loc.anchor ~= nil, "Anchor set failed")

        local pad = Padding.new(20.0)
        pad.left = 10.0
        assert(pad.left == 10.0 and pad.right == 20.0, "Padding failed")
    "##;

    lua.load(script).exec()?;
    Ok(())
}

#[test]
fn test_lua_components_creation() -> mlua::Result<()> {
    let lua = create_lua_instance()?;

    let script = r##"
        local txt = Text.new("Lua Engine Test")
        txt:set_size(36)
        txt:set_color("#FF0000")
        txt:set_weight(Weight.bold())

        assert(txt.content == "Lua Engine Test", "Text content failed")

        local col = Column.new(Size.youtube_video())
        col:set_arrangment(Arrangement.center())
        col:set_alignment(Alignment.center())

        local row = Row.new(Size.new(400, 100))
        row:push(txt)
        row:push(Text.new("Item 2"))
        col:push(row)

        local box = Container.new(row)
            :background("#000000")
            :border_color("#FFFFFF")
        box:set_border_width(2.0)
        box:set_border_radius(10.0)
        box:set_padding(Padding.new(15.0))

        local svg = SvgBox.new([[<svg width="50" height="50"><rect width="50" height="50" fill="red"/></svg>]])
        assert(svg ~= nil, "SvgBox creation failed")
    "##;

    lua.load(script).exec()?;
    Ok(())
}

#[test]
fn test_lua_model_creation_and_wait() -> mlua::Result<()> {
    let lua = create_lua_instance()?;

    let script = r##"
        local model = Model.new(Size.youtube_shorts(), FpsTarget.f30())
        assert(model.fps == 30, "Model fps failed")

        local txt = Text.new("Hello Cono")
        txt:set_size(40)
        txt:set_color("#FFFFFF")

        model:add_component(txt)
        model:wait(1.0) -- Should produce 30 frames

        local frame_count = model:scene().frames_count
        assert(frame_count == 30, "Expected 30 frames, got " .. tostring(frame_count))
        return model
    "##;

    let model: Model = lua.load(script).eval()?;
    assert_eq!(model.scene().frames_count(), 30);
    Ok(())
}

#[test]
fn test_lua_spawn_and_animation() -> mlua::Result<()> {
    let lua = create_lua_instance()?;

    let script = r##"
        local model = Model.new(Size.youtube_shorts(), FpsTarget.f120())

        local txt = Text.new("Animated Text")
        txt:set_size(32)
        txt:set_color("#00FF00")

        local start_loc = Location.new(
            Coordinate.with_progress(Coordinate.percentage(0.0), Coordinate.percentage(0.5)),
            Coordinate.with_progress(Coordinate.percentage(0.0), Coordinate.percentage(0.5))
        )

        local pointer = model:spawn_located(txt, start_loc)
        model:with_pointer(pointer, function(comp)
            comp:set_anchor(Anchor.center())
        end)

        model:animate_pointer(pointer, 1.0, Easing.ease_out(), function(comp , progress)
            comp:update_progress(progress)
        end)

        model:wait(0.5)
        return model
    "##;

    let model: Model = lua.load(script).eval()?;
    // 1.0s anim (30 frames) + 0.5s wait (15 frames) = 45 frames
    model.render_video(Path::new("./tests/ee.mp4")).unwrap();
    // assert_eq!(model.scene().frames_count(), 45);
    Ok(())
}

#[test]
fn test_lua_scene_multiplication() -> mlua::Result<()> {
    let lua = create_lua_instance()?;

    let script = r##"
        local model1 = Model.new(Size.youtube_video(), FpsTarget.f30())
        model1:add_component(Text.new("Scene 1"))
        model1:wait(0.5) -- 15 frames

        local model2 = Model.new(Size.youtube_video(), FpsTarget.f30())
        model2:add_component(Text.new("Scene 2"))
        model2:wait(0.5) -- 15 frames

        local combined = model1 * model2
        return combined.frames_count
    "##;

    let frames_count: usize = lua.load(script).eval()?;
    assert_eq!(frames_count, 15);
    Ok(())
}

#[test]
fn test_lua_typst_integration() -> mlua::Result<()> {
    let lua = create_lua_instance()?;

    let script = r##"
        local pager = Pager.new():add("$ E = m c^2 $")
        local world = LuaWorld.new(pager)
        local svgs = world:to_svgs()
        assert(#svgs == 1, "Expected 1 compiled SVG equation")
        return svgs[1]
    "##;

    lua.load(script).exec()?;
    Ok(())
}

#[test]
fn test_lua_full_video_render() -> mlua::Result<()> {
    let lua = create_lua_instance()?;

    let output_path = "./tests/lua_rendered_video.mp4";
    if Path::new(output_path).exists() {
        std::fs::remove_file(output_path).ok();
    }

    let script = format!(
        r##"
        local model = Model.new(Size.youtube_shorts(), FpsTarget.f120())

        local svg = SvgBox.new([[<svg width="100" height="100"><circle cx="50" cy="50" r="40" fill="blue"/></svg>]])
        local card = Container.new(svg)
            :background("#1E1E2E")
            :border_color("#89B4FA")

        card:set_border_width(4.0)
        card:set_border_radius(16.0)
        card:set_padding(Padding.new(24.0))

        local loc = Location.new(
            Coordinate.with_progress(Coordinate.percentage(0.5), Coordinate.percentage(0.5)),
            Coordinate.with_progress(Coordinate.percentage(0.2), Coordinate.percentage(0.5))
        )

        local ptr = model:spawn_located(card, loc)
        model:with_pointer(ptr, function(c) c:set_anchor(Anchor.center()) end)

        model:animate_easing(ptr, 5, Easing.ease_out(), function(c , a)
            c:with_mut(
                function(o)
                    o:update_progress(a)
                end
            ) 
        end)

        model:wait(0.5)

        model:render_video("{}")
    "##,
        output_path
    );

    lua.load(&script).exec()?;

    assert!(
        Path::new(output_path).exists(),
        "Output MP4 file was not created"
    );
    Ok(())
}
