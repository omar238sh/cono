# Cono Lua API — Video Generation & Animation Tutorials

Welcome to the **Cono Lua API** tutorial series! Cono is a high-performance programmatic video composition and animation engine. Using Lua (or Luau), you can compose UI layouts, render vector graphics and Typst mathematical equations, build smooth interpolated animations, and render ready-to-publish MP4 videos.

---

## 📚 Lesson Index

| Lesson | Topic | Description |
| :--- | :--- | :--- |
| [Lesson 1: Getting Started](01_getting_started.md) | Basics & Video Export | Learn how to initialize models, configure dimensions & FPS, build text/container elements, and export your first MP4. |
| [Lesson 2: Layout & Styling](02_layout_and_styling.md) | Flex Layouts & Typst Integration | Master flexible UI layouts (`Row`, `Column`, `Container`), spacing, padding, and rendering formatted math equations via Typst (`LuaWorld`). |
| [Lesson 3: Positioning & Anchors](03_positioning_and_anchors.md) | Coordinate Systems & Spawning | Explore absolute vs. percentage positioning (`Coordinate`, `Location`), anchor points (`Anchor`), and element controllers (`spawn_located`). |
| [Lesson 4: Animation & Easings](04_animation_and_easings.md) | Interpolation & Easing Functions | Learn how to create smooth time-based animations (`animate_pointer`), apply easing curves (`Linear`, `EaseIn`, `EaseOut`, `EaseInOut`), and hold static frames (`wait`). |
| [Lesson 5: Advanced Composition](05_advanced_composition.md) | Multi-scene & Vector Graphics | Combine multiple scenes/models with operators (`model1 * model2`), handle custom SVG assets (`SvgBox`), and build a complete social media short video. |

---

## 🚀 Quick Reference Snippet

Here is a minimal complete Lua script to create and export a 3-second animated video:

```lua
-- 1. Create a model (1080x1920 Vertical Short @ 60 FPS)
local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

-- 2. Create title text
local title = Text.new("Hello, Cono Video!")
    :size(48)
    :color("#FFFFFF")
    :weight(Weight.bold())

-- 3. Spawn component at screen center with percentage coordinates
local start_x = Coordinate.percentage(0.5)
local start_y = Coordinate.percentage(0.1)
local end_y   = Coordinate.percentage(0.5)

local pointer = model:spawn_located(title, Location.new(
    start_x,
    Coordinate.with_progress(start_y, end_y)
))

-- Align element center to coordinate point
model:with_pointer(pointer, function(comp)
    comp:set_anchor(Anchor.center())
end)

-- 4. Animate title dropping into position over 1.5 seconds with EaseOut
model:animate_pointer(pointer, 1.5, Easing.ease_out(), function(comp, progress)
    comp.location:set_progress(progress)
end)

-- 5. Hold static scene for 1.5 seconds
model:wait(1.5)

-- 6. Render MP4 output file
model:render_video("output.mp4")
```

---

## 🛠️ API Cheat Sheet

### Canvas & FPS Presets
- **Canvas Sizes**: `Size.youtube_video()` (1920x1080), `Size.youtube_shorts()` (1080x1920), `Size.tiktok()` (1080x1920), `Size.instagram_reels()` (1080x1920), `Size.instagram_square()` (1080x1080), `Size.new(width, height)`.
- **FPS Targets**: `FpsTarget.f24()`, `FpsTarget.f30()`, `FpsTarget.f60()`, `FpsTarget.f120()`, `FpsTarget.custom(60)`.
- **Easing Curves**: `Easing.linear()`, `Easing.ease_in()`, `Easing.ease_out()`, `Easing.ease_in_out()`.
- **Anchors**: `Anchor.top_left()`, `Anchor.top_right()`, `Anchor.center()`, `Anchor.bottom_left()`, `Anchor.bottom_right()`.

---

## 💡 IDE & Lua Language Server (LSP) Type Definitions

For full autocompletion, field type checking, and inline documentation in VS Code / Neovim / LuaLS:
1. Include or link [lua_definitions.lua](file:///home/omar/Desktop/cono/src/render/lua/lua_definitions.lua) in your workspace settings under `Lua.workspace.library`.
2. All global types (`Model`, `Text`, `Container`, `Column`, `Row`, `SvgBox`, `Location`, `Coordinate`, `Anchor`, `Easing`, `FpsTarget`, `LuaWorld`, `Pager`, etc.) are annotated with `@class`, `@param`, `@return`, and `@field`.

