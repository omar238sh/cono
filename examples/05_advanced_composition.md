# Lesson 5: Advanced Scene Composition & Multi-Scene Workflows

In Lesson 5, you will learn advanced techniques for building full production-quality videos: incorporating vector SVG graphics (`SvgBox`), sequencing and layering multiple scenes using the multiplication operator (`*`), and structuring clean modular video scripts.

---

## 1. Vector Assets & Graphics (`SvgBox`)

Cono natively supports vector graphics rendering via `SvgBox`. You can instantiate SVGs either directly from an SVG code string or by loading external SVG files from disk:

### Loading SVG from File Path
```lua
local logo = SvgBox.from_path("assets/logo.svg")
    :scale(1.5)                           -- Scale vector size by 1.5x
    :padding(Padding.new(16.0))
```

### Creating Inline SVG Code
```lua
local star_svg = SvgBox.new([[
    <svg width="100" height="100" viewBox="0 0 100 100">
        <polygon points="50,5 64,36 98,36 70,57 81,91 50,70 19,91 30,57 2,36 36,36" fill="#F9E2AF"/>
    </svg>
]])
```

---

## 2. Multi-Scene Composition & Scene Concatenation (`*`)

In Cono, the multiplication operator (`*`) between two `Model` instances concatenates their timeline scenes sequentially or merges layers seamlessly:

```lua
local intro_scene = Model.new(Size.youtube_shorts(), FpsTarget.f60())
-- ... build intro scene ...

local main_scene = Model.new(Size.youtube_shorts(), FpsTarget.f60())
-- ... build main content scene ...

local outro_scene = Model.new(Size.youtube_shorts(), FpsTarget.f60())
-- ... build outro CTA scene ...

-- Sequence scenes sequentially into a complete video composition!
local full_video = intro_scene * main_scene * outro_scene

full_video:render_video("final_short.mp4")
```

If one scene has more frames than another, multiplying them extends or loops the background timeline naturally, making complex multi-track overlays simple to compose.

---

## 3. Modular Video Script Architecture

For larger video projects, break your video down into clean scene-generator functions:

```lua
function create_intro_scene()
    local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())
    -- ...
    return model
end

function create_content_scene()
    local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())
    -- ...
    return model
end
```

---

## Complete Working Example

Save this script as `lesson5.lua`:

```lua
-- =============================================================================
-- SCENE 1: Intro Title Card
-- =============================================================================
local function build_intro()
    local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

    local title_text = Text.new("CONO LUA ENGINE")
        :size(52)
        :color("#F38BA8")
        :weight(Weight.bold())

    local intro_box = Container.new(title_text)
        :background("#1E1E2E")
        :border_color("#F38BA8")
        :border_width(4.0)
        :border_radius(20.0)
        :padding(Padding.new(40.0))

    local loc = Location.new(
        Coordinate.percentage(0.5),
        Coordinate.with_progress(Coordinate.percentage(-0.2), Coordinate.percentage(0.5))
    )
    
    local ptr = model:spawn_located(intro_box, loc)
    model:with_pointer(ptr, function(c) c:set_anchor(Anchor.center()) end)

    model:animate_pointer(ptr, 1.0, Easing.ease_out(), function(c, p)
        c.location:set_progress(p)
    end)
    
    model:wait(1.5)
    return model
end

-- =============================================================================
-- SCENE 2: Main Vector Graphics Content
-- =============================================================================
local function build_content()
    local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

    -- Inline Vector Star Icon
    local star = SvgBox.new([[
        <svg width="120" height="120" viewBox="0 0 100 100">
            <polygon points="50,5 64,36 98,36 70,57 81,91 50,70 19,91 30,57 2,36 36,36" fill="#FAB387"/>
        </svg>
    ]])

    local label = Text.new("Vector Graphics & Typst")
        :size(36)
        :color("#CDD6F4")

    local col = Column.new(Size.new(800, 400))
        :arrangment(Arrangement.center())
        :alignment(Alignment.center())
    col:push(star)
    col:push(label)

    local card = Container.new(col)
        :background("#181825")
        :border_radius(24.0)
        :padding(Padding.new(30.0))

    local ptr = model:spawn_located(card, Location.new(
        Coordinate.percentage(0.5),
        Coordinate.percentage(0.5)
    ))
    model:with_pointer(ptr, function(c) c:set_anchor(Anchor.center()) end)

    model:wait(3.0)
    return model
end

-- =============================================================================
-- SCENE 3: Outro Call-To-Action
-- =============================================================================
local function build_outro()
    local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

    local cta = Text.new("Start Building Videos with Lua!")
        :size(42)
        :color("#A6E3A1")
        :weight(Weight.bold())

    local box = Container.new(cta)
        :background("#11111B")
        :border_color("#A6E3A1")
        :border_width(4.0)
        :border_radius(16.0)
        :padding(Padding.new(32.0))

    local ptr = model:spawn_located(box, Location.new(
        Coordinate.percentage(0.5),
        Coordinate.percentage(0.5)
    ))
    model:with_pointer(ptr, function(c) c:set_anchor(Anchor.center()) end)

    model:wait(2.5)
    return model
end

-- =============================================================================
-- MAIN PIPELINE: Concatenate & Render
-- =============================================================================
print("Building scenes...")
local intro   = build_intro()
local content = build_content()
local outro   = build_outro()

-- Combine scenes sequentially using multiplication
local full_video = intro * content * outro

print("Rendering complete video composition to final_production.mp4...")
full_video:render_video("final_production.mp4")
print("Successfully rendered final_production.mp4!")
```

---

## 🏆 Summary

Congratulations! You have completed the Cono Lua API video generation series. You now possess all the tools to programmatically build dynamic, beautifully styled video animations and motion graphics with Lua.
