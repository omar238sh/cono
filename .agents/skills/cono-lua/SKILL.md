---
name: cono-lua
description: Guide and instructions for writing, debugging, animating, and rendering programmatic MP4 videos in Cono using Lua scripts, layout components, Typst math markup, and Lua LS annotations.
---

# Cono Lua Video Generation & Animation Skill

This skill equips AI agents with complete knowledge to generate, layout, animate, and render video content using Cono's Lua API.

---

## 1. Core Architecture

In Cono, every video composition centers around a `Model`:
- **Canvas Resolution (`Size`)**: Standard presets like `Size.youtube_video()` (1920x1080) or `Size.youtube_shorts()` (1080x1920), or custom resolution `Size.new(w, h)`.
- **Target Frame Rate (`FpsTarget`)**: Presets like `FpsTarget.f30()`, `FpsTarget.f60()`, `FpsTarget.f120()`, or `FpsTarget.custom(fps)`.
- **Timeline & Scene (`Scene`)**: Frames accumulated through component rendering, step-by-step keyframe animation, or static waiting (`model:wait(seconds)`).

---

## 2. Type Definitions & LSP Support

All Cono Lua global types, functions, methods, fields, and annotations are defined in:
`src/render/lua/lua_definitions.lua`

When inspecting or generating code, reference `lua_definitions.lua` for accurate type signatures (`@class`, `@param`, `@return`, `@field`).

---

## 3. UI Component Primitives

### Text (`Text`)
Fluently configurable text element:
```lua
local text = Text.new("Hello Cono")
    :size(48)                      -- Font size in px
    :color("#FFFFFF")              -- Hex color string
    :font("Inter")                 -- Font family name
    :weight(Weight.bold())         -- Font weight preset (Weight.bold(), Weight.medium(), etc.)
    :italic()                      -- Make font style italic
```

### Container (`Container`)
Wraps a single child component with styling:
```lua
local card = Container.new(text)
    :background("#1E1E2E")         -- Hex string or RGBA table {r, g, b, a}
    :border_color("#89B4FA")
    :border_width(4.0)
    :border_radius(16.0)
    :padding(Padding.new(32.0))    -- Uniform padding
```

### Layouts: `Column` & `Row`
Vertical and horizontal flexbox containers:
```lua
local col = Column.new(Size.youtube_video())
    :arrangment(Arrangement.center())  -- Main axis
    :alignment(Alignment.center())     -- Cross axis
    :padding(Padding.new(40.0))

col:push(Text.new("Title"):size(40):color("#FFFFFF"))
col:push(Text.new("Subtitle"):size(24):color("#A6ADC8"))
```

### Vector Graphics (`SvgBox`)
Renders inline SVG markup or external SVG files:
```lua
-- From inline SVG string
local icon = SvgBox.new([[<svg width="50" height="50"><circle cx="25" cy="25" r="20" fill="red"/></svg>]])

-- From file path
local logo = SvgBox.from_path("assets/logo.svg"):scale(1.5)
```

---

## 4. Positioning, Coordinates & Anchors

### Coordinates (`Coordinate`)
- **Absolute**: `Coordinate.absolute(200.0)` (200px)
- **Percentage**: `Coordinate.percentage(0.5)` (50% screen ratio)
- **Interpolated**: `Coordinate.with_progress(start_coord, end_coord)`

### Location & Anchors (`Location` & `Anchor`)
```lua
local loc = Location.new(
    Coordinate.percentage(0.5), -- X = 50%
    Coordinate.percentage(0.5)  -- Y = 50%
)
loc.anchor = Anchor.center()    -- Align element center to coordinate
```

### Spawning & Pointers (`spawn_located`)
```lua
local ptr = model:spawn_located(card, loc)

-- Mutate element in-place via pointer handle
model:with_pointer(ptr, function(comp)
    comp:set_anchor(Anchor.center())
end)
```

---

## 5. Animation & Timeline

### Time-Based Keyframe Animation (`animate_pointer`)
```lua
model:animate_pointer(
    ptr,                   -- ElementPointer
    1.5,                   -- Duration in seconds
    Easing.ease_out(),     -- Easing curve: linear(), ease_in(), ease_out(), ease_in_out()
    function(comp, progress)
        -- comp is LocatedComponent
        -- progress is eased_t from 0.0 to 1.0
        comp.location:set_progress(progress)
    end
)
```

### Holding Static State (`wait`)
```lua
model:wait(2.0) -- Hold scene state for 2 seconds
```

---

## 6. Typst Document & Math Integration

Compile formatted Typst math formulas directly to vector `SvgBox` elements:
```lua
local pager = Pager.new()
    :add("$ E = m c^2 $")
    :add("$ f(x) = integral_(-oo)^(+oo) e^(-x^2) d x $")

local world = LuaWorld.new(pager)
local svgs = world:to_svgs() -- Array of SvgBox elements

-- Add compiled math SVG to scene layout
model:add_component(svgs[1]:scale(2.0))
```

---

## 7. Multi-Scene Composition (`*`)

Multiply models to sequence scenes sequentially or layer timelines:
```lua
local intro = build_intro_model()
local main  = build_main_model()
local outro = build_outro_model()

local full_video = intro * main * outro
full_video:render_video("output.mp4")
```

---

## 8. Workflow Checklist for AI Agents

1. **Initialize canvas**: Always set target resolution `Size` and `FpsTarget`.
2. **Setup layout**: Use `Column`, `Row`, or `Container` for structural elements.
3. **Position elements**: Use `spawn_located` with percentage or absolute `Coordinate`s.
4. **Animate**: Call `animate_pointer` with appropriate `Easing` curves.
5. **Hold timeline**: Call `wait(seconds)` for pauses or reading time.
6. **Export**: Call `model:render_video("path/to/output.mp4")`.
