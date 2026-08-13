---
name: cono-lua
description: Exhaustive technical skill guide and master reference for programmatically generating, animating, composing, and rendering MP4 videos in Cono using Lua scripts, UI layout primitives, Typst math integration, transformation matrices (rotation & opacity), and Lua LS type annotations.
---

# Cono Programmatic Video & Animation Engine — Lua Master Skill

This skill equips AI agents with complete, production-grade knowledge to design, construct, animate, and render video content programmatically using Cono's Lua API.

---

## 1. Architectural Overview & Rendering Engine

Cono compiles Lua scripts into high-performance video compositions using a multi-layer pipeline:
1. **Script Execution (`mlua`)**: Lua scripts interact with Rust types via UserData proxies.
2. **Layout Engine (`Taffy`)**: Computes flexbox and absolute spatial coordinates for UI components.
3. **GPU Render Engine (`Vello`)**: Vector-graphic engine rendering anti-aliased scenes, affine matrices (translation, rotation around anchor), layers, and opacity blending.
4. **Typst Engine (`typst`)**: In-memory Typst compiler generating vector SVG paths for mathematical markup and formatted typography.
5. **Video Encoder (`FFmpeg`)**: Converts rendered RGBA frame buffer arrays into H.264 MP4 streams.

---

## 2. Global Definitions & LSP Autocomplete

All Cono Lua global types, functions, methods, fields, and annotations are documented in:
`src/render/lua/lua_definitions.lua`

Language server support is enabled via LuaLS annotations (`@class`, `@field`, `@param`, `@return`).

---

## 3. Core Engine Primitives

### Model (`Model`)
The root canvas and timeline accumulator:
```lua
-- Create canvas model with target Size and FpsTarget
local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

-- Configuration & Accessors
model:set_size(Size.youtube_video())
local sz = model:size()            -- Returns Size object
local fps = model:fps()            -- Returns integer FPS value (e.g. 60)

-- Timeline operations
model:add_component(component)     -- Adds static root element
model:wait(2.5)                    -- Holds current scene state for 2.5 seconds
model:render_video("output.mp4")   -- Compiles scene into MP4 file
```

### Canvas Dimensions (`Size`)
Presets and custom dimensions:
- `Size.youtube_video()`: 1920 × 1080 (16:9 Widescreen)
- `Size.youtube_shorts()`: 1080 × 1920 (9:16 Vertical)
- `Size.tiktok()`: 1080 × 1920 (9:16 Vertical)
- `Size.instagram_reels()`: 1080 × 1920 (9:16 Vertical)
- `Size.instagram_portrait()`: 1080 × 1350 (4:5 Portrait)
- `Size.instagram_square()`: 1080 × 1080 (1:1 Square)
- `Size.x_landscape()`: 1200 × 675 (16:9 Social)
- `Size.new(width, height)`: Custom dimensions in pixels

```lua
local sz = Size.new(800, 600)
local ratio = sz:aspect_ratio()    -- Returns width / height (1.333...)
local scaled = sz * 2.0            -- Operators: Size * number, Size / number
```

### Frame Rate Targets (`FpsTarget`)
- `FpsTarget.f30()`: 30 FPS
- `FpsTarget.f45()`: 45 FPS
- `FpsTarget.f60()`: 60 FPS
- `FpsTarget.f120()`: 120 FPS
- `FpsTarget.custom(fps)`: Custom integer frame rate

---

## 4. Component Primitives & Styling

### Text Component (`Text`)
Chainable fluent builder & mutator for typography:
```lua
-- Option 1: Fluent builder method chaining on creation
local title = Text.new("Cono Video Engine")
    :size(54)                      -- Font size in pixels
    :color("#F5E0DC")              -- Hex string (#RGB, #RRGGBB, #RRGGBBAA)
    :font("DejaVu Sans")           -- Font family name
    :weight(Weight.bold())         -- Font weight preset
    :italic()                      -- Enable italic style

-- Option 2: In-place mutator methods after creation
local subtitle = Text.new("Subtext")
subtitle:set_size(32)
subtitle:set_color("#A6ADC8")
subtitle:set_weight(Weight.medium())
subtitle:set_italic()

-- Option 3: Raw Typst Code / Math equations with Text.from_raw
local formula = Text.from_raw("$ e^(i pi) + 1 = 0 $")
    :size(48)
    :color("#F5E0DC")

-- Weight presets:
-- Weight.thin(), Weight.extra_light(), Weight.light(), Weight.regular(),
-- Weight.medium(), Weight.semi_bold(), Weight.bold(), Weight.extra_bold(), Weight.black()

-- FontStyle presets:
-- FontStyle.regular(), FontStyle.italic(), FontStyle.oblique()
```

> [!NOTE]
> **Agent Best Practice for Text & Typst Math**:
> - Use `Text.new("plain text")` for normal text elements.
> - Use `Text.from_raw("$ ... $")` when passing raw Typst code or math formulas. It automatically appends `#set page(width: auto, height: auto, fill: none, margin: 0.4cm)` to ensure transparent background.
> - Use fluent method chaining on `Text.new("..."):size(N):color(...)` during inline construction.
> - When modifying an existing `Text` variable, call in-place mutator methods like `text:set_size(N)`, `text:set_color(...)`, `text:set_weight(...)`, and `text:set_font(...)`.


### Container Box (`Container`)
Wraps a single child element with background, border, radius, and padding:
```lua
local card = Container.new(title)
    :background("#1E1E2E")         -- Hex string or RGBA table {r, g, b, a}
    :border_color("#89B4FA")
    :border_width(4.0)             -- Border width in px
    :border_radius(20.0)           -- Corner radius in px
    :padding(Padding.new(32.0))    -- Uniform padding

-- Size override:
card:with_size(Size.new(600, 200))
card:set_size(Size.new(600, 200))

-- In-place property setters:
card:set_background({30, 30, 46, 255})
card:set_border_color("#F38BA8")
card:set_border_width(6.0)
card:set_border_radius(24.0)
card:set_padding(Padding.new(24.0))
```

### Flexbox Layouts: `Column` & `Row`
Vertical (`Column`) and horizontal (`Row`) flex containers:
```lua
local col = Column.new(Size.youtube_video())
    :arrangment(Arrangement.center())   -- Main-axis arrangement (NOTE spelling: arrangment)
    :alignment(Alignment.center())      -- Cross-axis alignment
    :padding(Padding.new(40.0))

col:push(Text.new("Header Title"):size(40):color("#FFFFFF"))
col:push(Text.new("Subtitle details"):size(24):color("#A6ADC8"))

local row = Row.new(Size.new(800, 150))
    :arrangment(Arrangement.space_between())
    :alignment(Alignment.center())

row:push(Text.new("Left"):size(28):color("#FF5555"))
row:push(Text.new("Right"):size(28):color("#55FF55"))

col:push(row) -- Nesting Row inside Column
```

### 2D Grid Layout: `Grid`
Matrix 2D grid container supporting column counts, row counts, and column/row pixel gaps (taking `Column` as structural reference):
```lua
-- Create a 3-column Grid layout (600x400)
local grid = Grid.new(Size.new(600, 400), 3)
    :gap(16.0)                           -- Set uniform column & row gap in px
    :arrangment(Arrangement.center())   -- Main-axis arrangement
    :alignment(Alignment.center())      -- Cross-axis alignment
    :padding(Padding.new(24.0))

-- Push items (automatically flows across columns and rows)
grid:push(Text.new("Card 1"))
grid:push(Text.new("Card 2"))
grid:push(Text.new("Card 3"))
grid:push(Text.new("Card 4"))

-- In-place mutators:
grid:set_cols(4)
grid:set_gap(20.0)
grid:set_col_gap(24.0)
grid:set_row_gap(12.0)
```

#### Main-Axis `Arrangement` Presets:
- `Arrangement.start()`: Align at start
- `Arrangement.center()`: Centered along main axis
- `Arrangement.end()`: Align at end
- `Arrangement.space_between()`: Distribute space between children
- `Arrangement.space_around()`: Distribute space around children
- `Arrangement.space_evenly()`: Equal spacing around elements and edges
- `Arrangement.spaced_by(pixels)`: Fixed pixel gap between elements (e.g. `16`)

#### Cross-Axis `Alignment` Presets:
- `Alignment.start()`: Align at cross-axis start
- `Alignment.center()`: Cross-axis center
- `Alignment.end()`: Align at cross-axis end

### Padding (`Padding`)
Uniform or asymmetric spatial padding:
```lua
local pad_uniform = Padding.new(24.0)

local pad_custom = Padding.new(0.0)
    :left(20.0)
    :right(20.0)
    :top(40.0)
    :bottom(10.0)

pad_custom:edit(30.0) -- Modifies all sides in-place
```

### Vector SVG Box (`SvgBox`)
Renders vector graphics from raw SVG code or external files:
```lua
-- From SVG code string
local star = SvgBox.new([[
    <svg width="100" height="100" viewBox="0 0 100 100">
        <polygon points="50,5 64,36 98,36 70,57 81,91 50,70 19,91 30,57 2,36 36,36" fill="#F9E2AF"/>
    </svg>
]])

-- From file path
local logo = SvgBox.from_path("assets/logo.svg")
    :scale(1.5)                    -- Scale multiplier
    :padding(Padding.new(12.0))

logo:set_scale(2.0)                -- In-place scale setter
```

### Unconstrained Canvas (`FreeBox`)
Canvas container for absolute component placement:
```lua
local freebox = FreeBox.new(Size.youtube_video())
freebox:add(located_component)
```

---

## 5. Spatial Positioning, Transformations & Rotations

### Coordinates (`Coordinate`)
- **Absolute**: `Coordinate.absolute(250.0)` (250 pixels)
- **Percentage**: `Coordinate.percentage(0.5)` (50% canvas ratio)
- **Zero**: `Coordinate.zero()` (0 pixels)
- **Interpolated**: `Coordinate.with_progress(start_coord, end_coord)`

```lua
local start_x = Coordinate.percentage(0.0)
local end_x   = Coordinate.percentage(0.5)
local animated_x = Coordinate.with_progress(start_x, end_x)

-- Coordinate operations
animated_x:update_progress(0.5)    -- Sets progress to 50%
local scaled = start_x:scale(2.0)
local abs_num = end_x:to_num(1920) -- Resolves percentage against canvas max length (returns 960.0)
```

### Reference Anchors (`Anchor`)
Determines which point on the component aligns to its coordinate:
- `Anchor.top_left()`: Top-left corner
- `Anchor.top_right()`: Top-right corner
- `Anchor.center()`: Geometric center
- `Anchor.bottom_left()`: Bottom-left corner
- `Anchor.bottom_right()`: Bottom-right corner

### Location (`Location`)
Combines X & Y coordinates with an Anchor point:
```lua
-- Center of screen with percentage coordinates
local loc = Location.new(
    Coordinate.percentage(0.5),
    Coordinate.percentage(0.5)
)
loc.anchor = Anchor.center()

-- Interpolated Location between two locations
local start_loc = Location.new(Coordinate.percentage(0.5), Coordinate.percentage(-0.2))
local end_loc   = Location.new(Coordinate.percentage(0.5), Coordinate.percentage(0.5))

local animated_loc = Location.with_progress(start_loc, end_loc)
animated_loc:update_progress(0.5) -- Updates progress of X and Y coordinates
```

### Opacity (`Opacity`)
Controls transparency blending (0.0 transparent to 1.0 opaque):
```lua
-- Absolute opacity
local op_solid = Opacity.new(1.0)

-- Interpolated opacity fade
local op_fade = Opacity.with_progress(0.0, 1.0)
op_fade:update_progress(0.75)
local current_val = op_fade:value() -- Returns 0.75
```

### Rotation Angle (`Angle`)
Controls rotation transformation around component center (in degrees):
```lua
-- Absolute angle
local ang_flat = Angle.new(0.0)

-- Interpolated rotation (e.g. 0 to 360 degrees)
local ang_spin = Angle.with_progress(0.0, 360.0)
ang_spin:update_progress(0.5)
```

### LocatedComponent (`LocatedComponent`)
Wraps any component with spatial location, anchor, opacity, and rotation angle:
```lua
local text_comp = Text.new("Badge"):size(36):color("#FFFFFF")
local located = LocatedComponent.new(text_comp)
    :location(loc)
    :anchor(Anchor.center())
    :opacity(Opacity.with_progress(0.0, 1.0))
    :angle(Angle.with_progress(0.0, 360.0))

-- Unified progress update (updates location, opacity, and angle simultaneously!)
located:update_progress(0.5)

-- In-place setters:
located:set_location(new_loc)
located:set_anchor(Anchor.top_left())
located:set_opacity(Opacity.new(0.8))
located:set_angle(Angle.new(45.0))
located:shift(Location.new(Coordinate.absolute(10.0), Coordinate.absolute(20.0)))
```

---

## 6. Element Pointers & Scope Mutability

Components added to a `Model` with spatial locations are managed via **`ElementPointer`** handles.

### Spawning & Pointers (`spawn_located`)
```lua
local card = Container.new(Text.new("Animated Card"))

local entry_loc = Location.with_progress(
    Location.new(Coordinate.percentage(0.5), Coordinate.percentage(-0.2)),
    Location.new(Coordinate.percentage(0.5), Coordinate.percentage(0.5))
)

-- spawn_located registers component and returns an ElementPointer handle
local pointer = model:spawn_located(card, entry_loc)

-- Inspect or mutate component in-place using with_pointer
model:with_pointer(pointer, function(comp)
    comp:set_anchor(Anchor.center())
    comp:set_opacity(Opacity.with_progress(0.0, 1.0))
    comp:set_angle(Angle.with_progress(-180.0, 0.0))
end)
```

---

## 7. Animation, Keyframes & Easing Curves

### Easing Curves (`Easing`)
Easing curves control acceleration and deceleration:
- `Easing.linear()`: Constant speed
- `Easing.ease_in()`: Accelerates from rest
- `Easing.ease_out()`: Decelerates to a smooth stop
- `Easing.ease_in_out()`: Accelerates then decelerates

```lua
local curve = Easing.ease_out()
local eased_progress = curve:ease(0.5) -- Evaluates curve at t = 0.5
```

### Pointer Animation (`animate_pointer`)
`animate_pointer` steps through all frames required for the duration (`duration_sec * fps`) automatically:
```lua
model:animate_pointer(
    pointer,               -- ElementPointer handle
    2.0,                   -- Duration in seconds
    Easing.ease_out(),     -- Easing curve
    function(comp, progress)
        -- comp: LocatedComponent reference
        -- progress: Eased progression ratio (0.0 to 1.0)
        -- Calling update_progress updates Location, Opacity, and Angle simultaneously!
        comp:update_progress(progress)
    end
)
```

### Multi-Stage Keyframe Animation Pattern
```lua
-- Stage 1: Entrance (Fade-in + Slide-down + Rotate into place over 1.2s)
model:animate_pointer(ptr, 1.2, Easing.ease_out(), function(c, p)
    c:update_progress(p)
end)

-- Stage 2: Pause static view for 2.0s
model:wait(2.0)

-- Stage 3: Setup Exit Motion Path
model:with_pointer(ptr, function(c)
    c.location = Location.with_progress(
        Location.new(Coordinate.percentage(0.5), Coordinate.percentage(0.5)),
        Location.new(Coordinate.percentage(0.5), Coordinate.percentage(1.2))
    )
    c.opacity = Opacity.with_progress(1.0, 0.0)
end)

-- Stage 4: Exit (Slide-down + Fade-out over 0.8s)
model:animate_pointer(ptr, 0.8, Easing.ease_in(), function(c, p)
    c:update_progress(p)
end)
```

---

## 8. Typst Document & Math Engine Integration

Cono embeds the **Typst** compiler natively to render math equations, matrices, integrals, and formatted typography directly into vector `SvgBox` elements.

### Using `Pager` & `LuaWorld`
```lua
-- 1. Initialize Pager and add Typst markup snippets
local pager = Pager.new()
    :add("$ E = m c^2 $")
    :add("$ f(x) = integral_(-oo)^(+oo) e^(-x^2) d x = sqrt(pi) $")
    :add("$ A = mat(1, 2; 3, 4) $")

-- 2. Compile Typst document into array of SvgBox elements
local world = LuaWorld.new(pager)
local svgs = world:to_svgs() -- Returns table array of SvgBox objects (1-indexed!)

-- svgs[1] is the E = mc^2 equation SVG
-- svgs[2] is the Gaussian integral SVG
-- svgs[3] is the 2x2 Matrix SVG

local math_box = svgs[2]:scale(2.5) -- Scale equation vector 2.5x

model:add_component(math_box)
```

---

## 9. Multi-Scene Composition & Scene Arithmetic (`+` and `*`)

Cono supports scene arithmetic on `Model` and `Scene` instances:
- **`+` (Sequential Timeline Addition)**: Appends scenes end-to-end sequentially along the timeline.
- **`*` (Parallel Layer Multiplication)**: Overlays frames of scenes in parallel on top of each other.

```lua
-- Create scene 1 (Intro: 2.0s)
local intro = Model.new(Size.youtube_shorts(), FpsTarget.f60())
intro:add_component(Text.new("Welcome to Cono Engine"):size(44))
intro:wait(2.0)

-- Create scene 2 (Main Content: 4.0s)
local main_content = Model.new(Size.youtube_shorts(), FpsTarget.f60())
main_content:add_component(Text.from_raw("$ e^(i pi) + 1 = 0 $"):size(56))
main_content:wait(4.0)

-- Create scene 3 (Outro: 2.0s)
local outro = Model.new(Size.youtube_shorts(), FpsTarget.f60())
outro:add_component(Text.new("Subscribe for More!"):size(40))
outro:wait(2.0)

-- 1. Sequential Addition (+): Concatenates timelines (2s + 4s + 2s = 8s total)
local sequence = intro + main_content + outro
sequence:render_video("full_timeline.mp4") -- Both :render() and :render_video() are supported on Scene!

-- 2. Parallel Overlay (*): Merges layers across the same time duration
local overlay = intro * main_content
```

---

## 10. Complete Production Recipes

### Recipe 1: Rotating Vector Logo & Math Callout Video
```lua
local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

-- 1. Build Typst Math Equation
local pager = Pager.new():add("$ e^(i pi) + 1 = 0 $")
local world = LuaWorld.new(pager)
local math_svg = world:to_svgs()[1]:scale(2.5)

-- 2. Build Card Container
local title = Text.new("Euler's Identity"):size(44):color("#F5E0DC"):weight(Weight.bold())

local col = Column.new(Size.new(900, 500))
    :arrangment(Arrangement.center())
    :alignment(Alignment.center())
    :padding(Padding.new(30.0))
col:push(title)
col:push(math_svg)

local card = Container.new(col)
    :background("#1E1E2E")
    :border_color("#89B4FA")
    :border_width(5.0)
    :border_radius(28.0)
    :padding(Padding.new(40.0))

-- 3. Define Entry Location, Opacity, and Angle
local start_loc = Location.new(Coordinate.percentage(0.5), Coordinate.percentage(-0.3))
local end_loc   = Location.new(Coordinate.percentage(0.5), Coordinate.percentage(0.5))

local loc = Location.with_progress(start_loc, end_loc)
local ptr = model:spawn_located(card, loc)

model:with_pointer(ptr, function(c)
    c:set_anchor(Anchor.center())
    c:set_opacity(Opacity.with_progress(0.0, 1.0))
    c:set_angle(Angle.with_progress(-90.0, 0.0))
end)

-- 4. Animate Entrance: 1.5 seconds EaseOut
model:animate_pointer(ptr, 1.5, Easing.ease_out(), function(c, p)
    c:update_progress(p)
end)

-- 5. Hold static scene for 3.0 seconds
model:wait(3.0)

-- 6. Render MP4 output
model:render_video("eulers_identity.mp4")
print("Successfully rendered eulers_identity.mp4!")
```

---

## 11. AI Agent Rules & Anti-Patterns Checklist

1. **Lua 1-based Indexing**: Array results like `world:to_svgs()` are 1-indexed in Lua (`svgs[1]`, NOT `svgs[0]`).
2. **Correct Arrangement Spelling**: Method name on `Column` and `Row` is `arrangment` (e.g. `col:arrangment(Arrangement.center())`).
3. **Use Unified `update_progress`**: Always call `comp:update_progress(progress)` inside `animate_pointer` callbacks to automatically step Location, Opacity, and Angle progress.
4. **Pointer Mutation**: Use `model:with_pointer(pointer, callback)` to inspect or modify spawned components before or during animation sequences.
5. **Multi-Scene Sequential Multiplication**: Use `model1 * model2 * model3` to combine scenes cleanly without variable pollution.
6. **Output Path Verification**: Always conclude scripts with `model:render_video("filename.mp4")`.
