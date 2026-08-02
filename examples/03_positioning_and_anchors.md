# Lesson 3: Positioning, Coordinates, & Anchor Points

In Lesson 3, you will explore Cono's spatial positioning system. You will learn how to place elements anywhere on canvas using absolute pixels or screen percentage ratios (`Coordinate`), position containers (`Location`), align reference points (`Anchor`), and manipulate element handles via `ElementPointer`.

---

## 1. Coordinates: `Coordinate`

Coordinates define spatial values along X and Y axes. Cono supports two coordinate modes:

### 1. Absolute Pixel Coordinates (`Coordinate.absolute`)
Explicit pixel distances from canvas edges:
```lua
local x_px = Coordinate.absolute(250.0) -- 250 pixels
```

### 2. Screen Percentage Coordinates (`Coordinate.percentage`)
Normalized ratios from `0.0` (0%) to `1.0` (100%):
```lua
local x_center = Coordinate.percentage(0.5) -- 50% (screen center X)
local y_top    = Coordinate.percentage(0.1) -- 10% from top
```

### 3. Animated Progress Coordinates (`Coordinate.with_progress`)
Binds a start coordinate and end coordinate so that setting a single progress ratio `t` (0.0 to 1.0) interpolates smoothly:
```lua
local start_y = Coordinate.percentage(0.0) -- Start at top (0%)
local end_y   = Coordinate.percentage(0.5) -- End at center (50%)

local animated_y = Coordinate.with_progress(start_y, end_y)
```

---

## 2. Location & Anchors: `Location` & `Anchor`

A `Location` combines X & Y coordinates with an `Anchor` point.

### `Anchor` Points
The `Anchor` determines which point on the component aligns to the `Location` coordinate:
- `Anchor.top_left()`: Top-left corner of the element.
- `Anchor.top_right()`: Top-right corner of the element.
- `Anchor.center()`: Geometric center of the element.
- `Anchor.bottom_left()`: Bottom-left corner.
- `Anchor.bottom_right()`: Bottom-right corner.

```lua
-- Create a center screen location
local loc = Location.new(
    Coordinate.percentage(0.5), -- X = 50%
    Coordinate.percentage(0.5)  -- Y = 50%
)
loc.anchor = Anchor.center()
```

---

## 3. Spawning Located Components (`spawn_located`)

When you add a component to a `Model` with specific positioning, use `model:spawn_located(component, location)`.

`spawn_located` returns an **`ElementPointer`** handle, which allows you to animate or mutate that specific element during the video timeline!

```lua
local text_comp = Text.new("Centered Badge"):size(36):color("#FFFFFF")

-- Spawn text at 50% X, 50% Y
local pointer = model:spawn_located(text_comp, Location.new(
    Coordinate.percentage(0.5),
    Coordinate.percentage(0.5)
))

-- Set anchor to Center via with_pointer
model:with_pointer(pointer, function(comp)
    comp:set_anchor(Anchor.center())
end)
```

---

## 4. Modifying Elements with `with_pointer`

The `with_pointer(pointer, callback)` method gives you direct access to mutate the underlying `LocatedComponent` instance in-place:

```lua
model:with_pointer(pointer, function(comp)
    -- Shift element position by 50 pixels down
    comp:shift(Location.new(Coordinate.absolute(0.0), Coordinate.absolute(50.0)))
end)
```

---

## Complete Working Example

Save this script as `lesson3.lua`:

```lua
-- 1. Initialize Short Video canvas (1080x1920)
local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

-- 2. Build Card Component
local card = Container.new(
    Text.new("Positioning Demo"):size(40):color("#11111B"):weight(Weight.bold())
):background("#89B4FA")
 :border_radius(20.0)
 :padding(Padding.new(30.0))

-- 3. Define screen center location with percentage coordinates
local center_loc = Location.new(
    Coordinate.percentage(0.5),
    Coordinate.percentage(0.5)
)

-- 4. Spawn card at center screen
local card_ptr = model:spawn_located(card, center_loc)

-- 5. Adjust anchor so card is perfectly centered on screen
model:with_pointer(card_ptr, function(comp)
    comp:set_anchor(Anchor.center())
end)

-- 6. Hold frame for 3 seconds
model:wait(3.0)

-- 7. Render output video
model:render_video("lesson3_output.mp4")
print("Rendered lesson3_output.mp4 successfully!")
```

---

## What's Next?
Proceed to [Lesson 4: Animation & Easings](04_animation_and_easings.md) to bring your scenes to life with smooth time-based interpolation and easing curves!
