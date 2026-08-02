# Lesson 4: Animation, Interpolation, & Easing Curves

In Lesson 4, you will master keyframe animation and motion graphics in Cono. You will learn how to animate element pointers over time, apply easing curves (`Easing`), interpolate progress values `t`, and compose smooth motion paths.

---

## 1. How Animation Works in Cono

When you invoke an animation method on a `Model`, Cono automatically calculates the total number of frames required based on your model's FPS setting (`duration_sec * fps`) and steps through them one by one.

On each frame step:
1. Cono evaluates the time progress `t` (from `0.0` at start to `1.0` at finish).
2. Passes `t` through your chosen **Easing** curve function to compute `eased_t`.
3. Calls your custom animation callback function.
4. Captures the frame into the video timeline.

---

## 2. Easing Curves (`Easing`)

Easing curves control acceleration and deceleration to make motion feel natural and polished:

| Easing | Preset | Description |
| :--- | :--- | :--- |
| **Linear** | `Easing.linear()` | Constant speed throughout the animation. |
| **Ease In** | `Easing.ease_in()` | Starts slow and accelerates towards the end. |
| **Ease Out** | `Easing.ease_out()` | Starts fast and decelerates to a smooth stop. |
| **Ease In-Out** | `Easing.ease_in_out()` | Accelerates in the middle and decelerates at the end. |

You can pass `Easing` objects or string names like `"ease_in"`, `"ease_out"`, `"ease_in_out"`, `"linear"`.

---

## 3. Animating Pointers: `animate_pointer`

`animate_pointer` is the primary method for animating spawned elements:

```lua
model:animate_pointer(
    pointer_handle,        -- ElementPointer returned by spawn_located
    duration_in_seconds,   -- e.g. 2.0 seconds
    easing_function,       -- e.g. Easing.ease_out()
    function(comp, progress)
        -- comp is the LocatedComponent being animated
        -- progress is eased_t from 0.0 to 1.0
        comp.location:set_progress(progress)
    end
)
```

---

## 4. Custom Motion & Coordinate Animation

Combining `Coordinate.with_progress` with `comp.location:set_progress(progress)` allows creating complex entrance and exit paths easily:

```lua
-- Create a path moving X from -200px (offscreen left) to center screen (50%)
local start_x = Coordinate.absolute(-200.0)
local end_x   = Coordinate.percentage(0.5)

local animated_loc = Location.new(
    Coordinate.with_progress(start_x, end_x),
    Coordinate.percentage(0.5)
)
animated_loc.anchor = Anchor.center()

-- Spawn element with animated location
local ptr = model:spawn_located(title_component, animated_loc)

-- Animate entrance over 1.2 seconds with EaseOut
model:animate_pointer(ptr, 1.2, Easing.ease_out(), function(comp, progress)
    comp.location:set_progress(progress)
end)

-- Hold in place for 2.0 seconds
model:wait(2.0)

-- Animate exit path (moving Y from 50% to 120% offscreen bottom)
model:with_pointer(ptr, function(comp)
    comp.location.y = Coordinate.with_progress(
        Coordinate.percentage(0.5),
        Coordinate.percentage(1.2)
    )
end)

model:animate_pointer(ptr, 1.0, Easing.ease_in(), function(comp, progress)
    comp.location:set_progress(progress)
end)
```

---

## Complete Working Example

Save this script as `lesson4.lua`:

```lua
-- 1. Create a 60 FPS YouTube Shorts Video Model (1080x1920)
local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

-- 2. Build Title Text Component
local title = Text.new("SMOOTH ANIMATIONS")
    :size(48)
    :color("#A6E3A1")
    :weight(Weight.bold())

local box = Container.new(title)
    :background("#181825")
    :border_color("#A6E3A1")
    :border_width(4.0)
    :border_radius(16.0)
    :padding(Padding.new(36.0))

-- 3. Define Entry Motion Path (From Top Offscreen to Screen Center)
local entry_loc = Location.new(
    Coordinate.percentage(0.5),
    Coordinate.with_progress(
        Coordinate.percentage(-0.2), -- Offscreen Top
        Coordinate.percentage(0.5)   -- Screen Center
    )
)

local box_ptr = model:spawn_located(box, entry_loc)

model:with_pointer(box_ptr, function(comp)
    comp:set_anchor(Anchor.center())
end)

-- 4. Animate Entrance: 1.5 seconds with EaseInOut
model:animate_pointer(box_ptr, 1.5, Easing.ease_in_out(), function(comp, progress)
    comp.location:set_progress(progress)
end)

-- 5. Pause static view for 1.0 second
model:wait(1.0)

-- 6. Setup Exit Motion Path (From Screen Center to Offscreen Bottom)
model:with_pointer(box_ptr, function(comp)
    comp.location.y = Coordinate.with_progress(
        Coordinate.percentage(0.5),
        Coordinate.percentage(1.2) -- Offscreen Bottom
    )
end)

-- 7. Animate Exit: 1.0 second with EaseIn
model:animate_pointer(box_ptr, 1.0, Easing.ease_in(), function(comp, progress)
    comp.location:set_progress(progress)
end)

-- 8. Render output video file
model:render_video("lesson4_output.mp4")
print("Rendered lesson4_output.mp4 successfully!")
```

---

## What's Next?
Proceed to [Lesson 5: Advanced Composition](05_advanced_composition.md) to combine multi-scene clips, load vector SVG graphics, and build complete short-form video content!
