# Lesson 2: Layout Systems, Styling, & Typst Math Integration

In Lesson 2, you will learn how to arrange multiple components using flexbox-like layouts (`Row` and `Column`), apply precise alignment and padding rules, and compile formatted mathematical equations via **Typst integration**.

---

## 1. Flexbox Layouts: `Row` & `Column`

Cono provides two layout primitives for organizing components:
- **`Row`**: Lays out child elements horizontally along a row.
- **`Column`**: Lays out child elements vertically along a column.

Both accept canvas size parameters and support child pushing via `:push()`.

### Creating a `Column` Layout

```lua
local col = Column.new(Size.youtube_video())

-- Add children into the vertical column
col:push(Text.new("Header Title"):size(40):color("#FFFFFF"))
col:push(Text.new("Subtitle text describing the video section"):size(24):color("#A6ADC8"))
```

### Creating a `Row` Layout

```lua
local row = Row.new(Size.new(800, 150))

row:push(Text.new("Left Item"):size(28):color("#FF5555"))
row:push(Text.new("Right Item"):size(28):color("#55FF55"))
```

---

## 2. Layout Alignment & Arrangement

You can fine-tune layout distribution using `Arrangement` (main-axis alignment) and `Alignment` (cross-axis alignment):

### `Arrangement` Options (Main Axis)
- `Arrangement.start()`: Align children at the beginning.
- `Arrangement.end()`: Align children at the end.
- `Arrangement.center()`: Center children along the main axis.
- `Arrangement.space_between()`: Distribute children evenly with space between them.
- `Arrangement.space_around()`: Distribute space around each child.
- `Arrangement.space_evenly()`: Equal spacing between every element and edge.
- `Arrangement.spaced_by(pixels)`: Fixed pixel gap between elements (e.g. `16`px).

### `Alignment` Options (Cross Axis)
- `Alignment.start()`: Cross-axis start alignment.
- `Alignment.center()`: Cross-axis center alignment.
- `Alignment.end()`: Cross-axis end alignment.

### Applying Arrangement & Alignment:

```lua
local main_col = Column.new(Size.youtube_video())
    :arrangment(Arrangement.center())   -- Vertically centered
    :alignment(Alignment.center())      -- Horizontally centered
    :padding(Padding.new(40.0))          -- 40px outer padding
```

---

## 3. Customizing `Padding`

The `Padding` type allows setting padding across all sides or individually:

```lua
-- Uniform padding on all 4 sides
local pad_all = Padding.new(24.0)

-- Asymmetric padding per side
local pad_custom = Padding.new(0.0)
    :left(20.0)
    :right(20.0)
    :top(40.0)
    :bottom(10.0)
```

---

## 4. Typst Mathematical & Document Integration

Cono embeds the **Typst** compiler engine natively! You can write standard Typst math syntax (such as `$E = m c^2$` or integrals) and render them directly to vector `SvgBox` elements.

### Using `Pager` & `LuaWorld`

```lua
-- 1. Create a Pager and add Typst math snippets
local pager = Pager.new()
    :add("$ F(omega) = integral_(-oo)^(+oo) f(t) dot e^(-i omega t) d t $")
    :add("$ x = (-b +- sqrt(b^2 - 4a c)) / (2a) $")

-- 2. Compile Typst snippets into Lua SvgBox objects
local world = LuaWorld.new(pager)
local svgs = world:to_svgs() -- Returns array/table of SvgBox objects

-- svgs[1] is the Fourier Transform equation box!
-- svgs[2] is the Quadratic Formula equation box!
```

---

## Complete Working Example

Save this script as `lesson2.lua`:

```lua
-- 1. Create 1080p Model @ 30 FPS
local model = Model.new(Size.youtube_video(), FpsTarget.f30())

-- 2. Compile Math Equations via Typst
local pager = Pager.new()
    :add("$ f(x) = sum_(n=0)^oo (f^(n)(a))/(n!) (x - a)^n $")

local world = LuaWorld::new(pager) -- Compiles Typst document
local math_svgs = world:to_svgs()
local taylor_eq = math_svgs[1]:scale(2.0) -- Scale equation vector up 2x

-- 3. Construct Layout Column
local layout = Column.new(Size.youtube_video())
    :arrangment(Arrangement.space_between())
    :alignment(Alignment.center())
    :padding(Padding.new(60.0))

-- Title
local title = Text.new("Taylor Series Expansion")
    :size(48)
    :color("#89B4FA")
    :weight(Weight.bold())

-- Add elements to layout
layout:push(title)
layout:push(taylor_eq)
layout:push(Text.new("Rendered live with Typst & Cono"):size(24):color("#A6ADC8"))

-- 4. Add layout component to model & wait
model:add_component(layout)
model:wait(5.0)

-- 5. Render output video
model:render_video("lesson2_output.mp4")
print("Rendered lesson2_output.mp4 successfully!")
```

---

## What's Next?
Proceed to [Lesson 3: Positioning & Anchors](03_positioning_and_anchors.md) to master absolute and percentage spatial positioning!
