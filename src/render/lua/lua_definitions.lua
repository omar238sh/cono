---@meta
--- Lua definitions and annotations for the Cono Video Rendering Engine.
--- Target language server: lua-language-server (Sumneko).

---@alias Color string|table<integer, integer> Hex color string (e.g., "#FF0000") or RGBA array table (e.g., {255, 0, 0, 255})
---@alias ComponentAny Text|Column|Row|Container|FreeBox|SvgBox|LocatedComponent|PlaceHolderText|PlaceHolderColumn|PlaceHolderRow|PlaceHolderContainer|PlaceHolderFreeBox|PlaceHolderSvgBox|PlaceHolderLocatedComponent

---=============================================================================
--- Size
---=============================================================================

---@class Size
---@field width number Canvas width in pixels
---@field height number Canvas height in pixels
Size = {}

--- Creates a new custom Size.
---@param width number Width in pixels
---@param height number Height in pixels
---@return Size
function Size.new(width, height) end

--- Preset for Widescreen Landscape (1200x675).
---@return Size
function Size.x_landscape() end

--- Preset for TikTok video (1080x1920).
---@return Size
function Size.tiktok() end

--- Preset for Instagram Reels (1080x1920).
---@return Size
function Size.instagram_reels() end

--- Preset for Instagram Portrait (1080x1350).
---@return Size
function Size.instagram_portrait() end

--- Preset for Instagram Square (1080x1080).
---@return Size
function Size.instagram_square() end

--- Preset for YouTube Shorts (1080x1920).
---@return Size
function Size.youtube_shorts() end

--- Preset for YouTube Video (1920x1080).
---@return Size
function Size.youtube_video() end

--- Returns aspect ratio (width / height).
---@return number
function Size:aspect_ratio() end


---=============================================================================
--- Text Weight & FontStyle
---=============================================================================

---@class Weight
Weight = {}

---@return Weight
function Weight.thin() end

---@return Weight
function Weight.extra_light() end

---@return Weight
function Weight.light() end

---@return Weight
function Weight.regular() end

---@return Weight
function Weight.medium() end

---@return Weight
function Weight.semi_bold() end

---@return Weight
function Weight.bold() end

---@return Weight
function Weight.extra_bold() end

---@return Weight
function Weight.black() end


---@class FontStyle
FontStyle = {}

---@return FontStyle
function FontStyle.regular() end

---@return FontStyle
function FontStyle.italic() end

---@return FontStyle
function FontStyle.oblique() end


---=============================================================================
--- Text
---=============================================================================

---@class Text
---@field content string Text content
---@field color string|nil Color hex string
---@field font string|nil Font family name
---@field size integer|nil Font size in pixels
Text = {}

--- Creates a new Text component.
---@param content string Text content
---@return Text
function Text.new(content) end

--- Sets text color (chainable).
---@param c string Hex color string
---@return Text
function Text:color(c) end

--- Sets text color in-place.
---@param c string Hex color string
function Text:set_color(c) end

--- Sets font family (chainable).
---@param f string Font family name
---@return Text
function Text:font(f) end

--- Sets font family in-place.
---@param f string Font family name
function Text:set_font(f) end

--- Sets font size (chainable).
---@param s integer Size in pixels
---@return Text
function Text:size(s) end

--- Sets font size in-place.
---@param s integer Size in pixels
function Text:set_size(s) end

--- Sets font weight (chainable).
---@param w Weight Font weight
---@return Text
function Text:weight(w) end

--- Sets font weight in-place.
---@param w Weight Font weight
function Text:set_weight(w) end

--- Sets italic style (chainable).
---@return Text
function Text:italic() end

--- Sets italic style in-place.
function Text:set_italic() end

--- Converts text component to Typst representation.
---@return string
function Text:to_typst() end


---=============================================================================
--- Padding & Alignment & Arrangement & Style
---=============================================================================

---@class Padding
---@field left number Left padding in pixels
---@field right number Right padding in pixels
---@field top number Top padding in pixels
---@field bottom number Bottom padding in pixels
Padding = {}

--- Creates uniform Padding on all sides.
---@param val number Padding value in pixels
---@return Padding
function Padding.new(val) end

--- Modifies all padding values in-place.
---@param val number Padding value in pixels
function Padding:edit(val) end

--- Sets left padding (chainable).
---@param val number
---@return Padding
function Padding:left(val) end

--- Sets right padding (chainable).
---@param val number
---@return Padding
function Padding:right(val) end

--- Sets top padding (chainable).
---@param val number
---@return Padding
function Padding:top(val) end

--- Sets bottom padding (chainable).
---@param val number
---@return Padding
function Padding:bottom(val) end


---@class Alignment
Alignment = {}

--- Align at start of cross-axis.
---@return Alignment
function Alignment.start() end

--- Align at end of cross-axis.
---@return Alignment
function Alignment.end() end

--- Center on cross-axis.
---@return Alignment
function Alignment.center() end


---@class Arrangement
Arrangement = {}

--- Align children at start.
---@return Arrangement
function Arrangement.start() end

--- Align children at end.
---@return Arrangement
function Arrangement.end() end

--- Center children along main axis.
---@return Arrangement
function Arrangement.center() end

--- Distribute children evenly with space between them.
---@return Arrangement
function Arrangement.space_between() end

--- Distribute space around each child.
---@return Arrangement
function Arrangement.space_around() end

--- Equal space between children and edges.
---@return Arrangement
function Arrangement.space_evenly() end

--- Fixed pixel spacing between elements.
---@param pixels integer Gap size in pixels
---@return Arrangement
function Arrangement.spaced_by(pixels) end


---@class Style
---@field size Size Container size
---@field arrangement Arrangement Main-axis arrangement
---@field alignment Alignment Cross-axis alignment
---@field padding Padding Outer padding
Style = {}

--- Creates a new Style object.
---@param size Size
---@return Style
function Style.new(size) end

---@return Size
function Style:size() end

---@return Arrangement
function Style:arrangement() end

---@param arr Arrangement
function Style:set_arrangement(arr) end

---@return Alignment
function Style:alignment() end

---@param align Alignment
function Style:set_alignment(align) end

---@return Padding
function Style:padding() end

---@param pad Padding
function Style:set_padding(pad) end


---=============================================================================
--- Column & Row
---=============================================================================

---@class Column
---@field size Size Container size
Column = {}

--- Creates a new vertical Column layout.
---@param size Size
---@return Column
function Column.new(size) end

--- Pushes a child component into the Column.
---@param val ComponentAny Child component
function Column:push(val) end

--- Sets main-axis arrangement (chainable).
---@param arr Arrangement
---@return Column
function Column:arrangment(arr) end

--- Sets main-axis arrangement in-place.
---@param arr Arrangement
function Column:set_arrangment(arr) end

--- Sets cross-axis alignment (chainable).
---@param align Alignment
---@return Column
function Column:alignment(align) end

--- Sets cross-axis alignment in-place.
---@param align Alignment
function Column:set_alignment(align) end

--- Sets padding (chainable).
---@param pad Padding
---@return Column
function Column:padding(pad) end

--- Sets padding in-place.
---@param pad Padding
function Column:set_padding(pad) end


---@class Row
---@field size Size Container size
Row = {}

--- Creates a new horizontal Row layout.
---@param size Size
---@return Row
function Row.new(size) end

--- Pushes a child component into the Row.
---@param val ComponentAny Child component
function Row:push(val) end

--- Sets main-axis arrangement (chainable).
---@param arr Arrangement
---@return Row
function Row:arrangment(arr) end

--- Sets main-axis arrangement in-place.
---@param arr Arrangement
function Row:set_arrangment(arr) end

--- Sets cross-axis alignment (chainable).
---@param align Alignment
---@return Row
function Row:alignment(align) end

--- Sets cross-axis alignment in-place.
---@param align Alignment
function Row:set_alignment(align) end

--- Sets padding (chainable).
---@param pad Padding
---@return Row
function Row:padding(pad) end

--- Sets padding in-place.
---@param pad Padding
function Row:set_padding(pad) end


---=============================================================================
--- Container
---=============================================================================

---@class Container
---@field size Size Box size
---@field border_width number Border width in pixels
---@field border_radius number Corner radius in pixels
---@field padding Padding Padding inside container
Container = {}

--- Wraps a child component in a Container.
---@param child ComponentAny Child component
---@return Container
function Container.new(child) end

--- Sets background color (chainable).
---@param color_val Color Color string or table
---@return Container
function Container:background(color_val) end

--- Sets background color in-place.
---@param color_val Color Color string or table
function Container:set_background(color_val) end

--- Sets border color (chainable).
---@param color_val Color Color string or table
---@return Container
function Container:border_color(color_val) end

--- Sets border color in-place.
---@param color_val Color Color string or table
function Container:set_border_color(color_val) end

--- Sets border width (chainable).
---@param width number Border width in pixels
---@return Container
function Container:border_width(width) end

--- Sets border width in-place.
---@param width number Border width in pixels
function Container:set_border_width(width) end

--- Sets border radius (chainable).
---@param radius number Radius in pixels
---@return Container
function Container:border_radius(radius) end

--- Sets border radius in-place.
---@param radius number Radius in pixels
function Container:set_border_radius(radius) end

--- Sets padding (chainable).
---@param padding Padding
---@return Container
function Container:padding(padding) end

--- Sets padding in-place.
---@param padding Padding
function Container:set_padding(padding) end

--- Overrides container size (chainable).
---@param size Size
---@return Container
function Container:with_size(size) end

--- Sets container size in-place.
---@param size Size
function Container:set_size(size) end


---=============================================================================
--- SvgBox
---=============================================================================

---@class SvgBox
---@field size Size SVG size
---@field padding Padding Inner padding
SvgBox = {}

--- Parses an SVG string into an SvgBox.
---@param content string Raw SVG content string
---@return SvgBox
function SvgBox.new(content) end

--- Loads an SVG from file path.
---@param path_str string Path to SVG file
---@return SvgBox
function SvgBox.from_path(path_str) end

--- Sets padding (chainable).
---@param pad Padding
---@return SvgBox
function SvgBox:padding(pad) end

--- Sets padding in-place.
---@param pad Padding
function SvgBox:set_padding(pad) end

--- Overrides size (chainable).
---@param size Size
---@return SvgBox
function SvgBox:with_size(size) end

--- Scales the SVG dimensions (chainable).
---@param factor number Scaling multiplier
---@return SvgBox
function SvgBox:scale(factor) end

--- Scales the SVG dimensions in-place.
---@param factor number Scaling multiplier
function SvgBox:set_scale(factor) end


---=============================================================================
--- Coordinate & Anchor & Location & LocatedComponent & FreeBox
---=============================================================================

---@class Coordinate
Coordinate = {}

--- Creates an absolute pixel coordinate.
---@param v number Pixel value
---@return Coordinate
function Coordinate.absolute(v) end

--- Creates a percentage coordinate (0.0 to 1.0).
---@param v number Fractional percentage
---@return Coordinate
function Coordinate.percentage(v) end

--- Zero coordinate (0 pixels).
---@return Coordinate
function Coordinate.zero() end

--- Interpolates coordinate from start to end based on animation progress.
---@param start Coordinate
---@param end_coord Coordinate
---@return Coordinate
function Coordinate.with_progress(start, end_coord) end

--- Scales coordinate (chainable).
---@param factor number
---@return Coordinate
function Coordinate:scale(factor) end

--- Converts to absolute coordinate against max length.
---@param max_len number
---@return Coordinate
function Coordinate:to_absolute(max_len) end

--- Evaluates coordinate to numeric pixel value.
---@param max_len number
---@return number
function Coordinate:to_num(max_len) end


---@class Anchor
Anchor = {}

---@return Anchor
function Anchor.center() end

---@return Anchor
function Anchor.top_left() end

---@return Anchor
function Anchor.top_right() end

---@return Anchor
function Anchor.bottom_left() end

---@return Anchor
function Anchor.bottom_right() end


---@class Location
---@field x Coordinate X position
---@field y Coordinate Y position
---@field anchor Anchor Anchor point
Location = {}

--- Creates a Location at x, y coordinates.
---@param x Coordinate
---@param y Coordinate
---@return Location
function Location.new(x, y) end

--- Zero location (0, 0).
---@return Location
function Location.zero() end

--- Center location with anchor.
---@param anchor Anchor
---@return Location
function Location.center(anchor) end

--- Scales location coordinates (chainable).
---@param factor number
---@return Location
function Location:scale(factor) end

--- Resolves location relative to top-left for container size.
---@param size Size
---@return Location
function Location:as_topleft(size) end

--- Updates animation progress (0.0 to 1.0) for interpolated coordinates.
---@param p number Progress fraction
function Location:set_progress(p) end


---@class LocatedComponent
---@field location Location Component location
LocatedComponent = {}

--- Wraps a component inside a LocatedComponent.
---@param val ComponentAny
---@return LocatedComponent
function LocatedComponent.new(val) end

--- Updates location progress value (0.0 to 1.0).
---@param progress number
function LocatedComponent:update_progress(progress) end

--- Sets location (chainable).
---@param loc Location
---@return LocatedComponent
function LocatedComponent:location(loc) end

--- Sets location in-place.
---@param loc Location
function LocatedComponent:set_location(loc) end

--- Sets anchor (chainable).
---@param anchor Anchor
---@return LocatedComponent
function LocatedComponent:anchor(anchor) end

--- Sets anchor in-place.
---@param anchor Anchor
function LocatedComponent:set_anchor(anchor) end

--- Shifts component by relative location delta.
---@param shift_loc Location
function LocatedComponent:shift(shift_loc) end

--- Synchronizes location internal state.
function LocatedComponent:sync_location() end

--- Returns current Location.
---@return Location
function LocatedComponent:get_location() end


---@class FreeBox
---@field size Size Canvas size
FreeBox = {}

--- Creates a new FreeBox container.
---@param size Size
---@return FreeBox
function FreeBox.new(size) end

--- Adds a located child component (chainable).
---@param child LocatedComponent
---@return FreeBox
function FreeBox:with_child(child) end

--- Adds a component into FreeBox in-place.
---@param val ComponentAny
function FreeBox:add(val) end


---=============================================================================
--- ElementPointers & PlaceHolders
---=============================================================================

---@class ElementPointer
ElementPointer = {}

--- Drops reference to component.
function ElementPointer:drop_component() end

--- Accesses component immutably inside scope closure.
---@param func fun(comp: any) Closure function
function ElementPointer:with(func) end

--- Accesses component mutably inside scope closure.
---@param func fun(comp: any) Closure function
function ElementPointer:with_mut(func) end


---@class PlaceHolderLocatedComponent
PlaceHolderLocatedComponent = {}
---@return ElementPointer Pointer to component
function PlaceHolderLocatedComponent:pointer() end

---@class PlaceHolderText
PlaceHolderText = {}
---@return ElementPointer Pointer to component
function PlaceHolderText:pointer() end

---@class PlaceHolderColumn
PlaceHolderColumn = {}
---@return ElementPointer Pointer to component
function PlaceHolderColumn:pointer() end

---@class PlaceHolderRow
PlaceHolderRow = {}
---@return ElementPointer Pointer to component
function PlaceHolderRow:pointer() end

---@class PlaceHolderContainer
PlaceHolderContainer = {}
---@return ElementPointer Pointer to component
function PlaceHolderContainer:pointer() end

---@class PlaceHolderFreeBox
PlaceHolderFreeBox = {}
---@return ElementPointer Pointer to component
function PlaceHolderFreeBox:pointer() end

---@class PlaceHolderSvgBox
PlaceHolderSvgBox = {}
---@return ElementPointer Pointer to component
function PlaceHolderSvgBox:pointer() end


---=============================================================================
--- Animation: FpsTarget & Easing
---=============================================================================

---@class FpsTarget
---@field value integer FPS target numeric value
FpsTarget = {}

--- Preset 30 FPS target.
---@return FpsTarget
function FpsTarget.f30() end

--- Preset 45 FPS target.
---@return FpsTarget
function FpsTarget.f45() end

--- Preset 60 FPS target.
---@return FpsTarget
function FpsTarget.f60() end

--- Preset 120 FPS target.
---@return FpsTarget
function FpsTarget.f120() end

--- Custom FPS target.
---@param fps integer Custom FPS rate
---@return FpsTarget
function FpsTarget.custom(fps) end

--- Returns FPS numeric value.
---@return integer
function FpsTarget:value() end


---@class Easing
Easing = {}

--- Linear animation curve.
---@return Easing
function Easing.linear() end

--- Ease-in animation curve.
---@return Easing
function Easing.ease_in() end

--- Ease-out animation curve.
---@return Easing
function Easing.ease_out() end

--- Ease-in-out animation curve.
---@return Easing
function Easing.ease_in_out() end

--- Evaluates easing curve at progress t (0.0 to 1.0).
---@param t number Progress fraction
---@return number Eased value
function Easing:ease(t) end


---=============================================================================
--- Frame & Scene & RenderedFrame
---=============================================================================

---@class Frame
---@field content FreeBox Root frame box
Frame = {}

--- Creates a new canvas Frame.
---@param size Size Frame size
---@return Frame
function Frame.new(size) end

--- Adds component to frame.
---@param val ComponentAny Component to add
function Frame:add(val) end


---@class RenderedFrame
---@field size Size Rendered frame size
RenderedFrame = {}

--- Returns size of rendered frame.
---@return Size
function RenderedFrame:size() end


---@class Scene
---@field fps integer Scene target FPS rate
---@field frames_count integer Total number of frames in scene
Scene = {}

--- Creates a new empty Scene.
---@return Scene
function Scene.new() end

--- Returns target FPS value.
---@return integer
function Scene:fps() end

--- Sets target FPS value.
---@param fps integer
function Scene:set_fps(fps) end

--- Returns number of frames in scene.
---@return integer
function Scene:frames_count() end

--- Pushes a frame into the scene.
---@param frame Frame
function Scene:push_frame(frame) end

--- Appends another scene to this scene.
---@param other Scene
function Scene:append(other) end

--- Renders scene to MP4 video file at specified path.
---@param path_str string Path to destination MP4 file
function Scene:render(path_str) end


---=============================================================================
--- Model (Main Animation & Composition Canvas)
---=============================================================================

---@class Model
---@field size Size Canvas dimensions
---@field fps integer Frame rate target
Model = {}

--- Creates a new Model animation canvas.
---@param size Size Canvas resolution
---@param fps_val FpsTarget Target frame rate
---@return Model
function Model.new(size, fps_val) end

--- Returns canvas resolution size.
---@return Size
function Model:size() end

--- Sets canvas resolution size.
---@param size Size
function Model:set_size(size) end

--- Returns target FPS.
---@return integer
function Model:fps() end

--- Sets target FPS.
---@param fps_val FpsTarget
function Model:set_fps(fps_val) end

--- Adds a static component to the model root.
---@param val ComponentAny Component to add
function Model:add_component(val) end

--- Spawns a component with a location and returns its ElementPointer.
---@param val ComponentAny Component to place
---@param loc Location Component location
---@return ElementPointer Pointer to spawned element
function Model:spawn_located(val, loc) end

--- Constructs a frame populated with all registered base components.
---@return Frame
function Model:create_frame() end

--- Pushes a pre-constructed frame directly into the underlying scene.
---@param frame Frame
function Model:push_frame(frame) end

--- Accesses the inner Scene representation.
---@return Scene
function Model:scene() end

--- Holds current state static for specified duration in seconds.
---@param duration_sec number Duration to hold in seconds
function Model:wait(duration_sec) end

--- Renders the animation model directly to an MP4 video file.
---@param path_str string Path to output video file
function Model:render_video(path_str) end

--- Animates a generic target object with linear easing.
---@param target any UserData object to animate
---@param duration_sec number Animation duration in seconds
---@param anim_fn fun(target: any, progress: number) Animation step callback
function Model:animate(target, duration_sec, anim_fn) end

--- Animates a generic target object with specified Easing curve.
---@param target any UserData object to animate
---@param duration_sec number Animation duration in seconds
---@param easing_val Easing Easing curve
---@param anim_fn fun(target: any, progress: number) Animation step callback
function Model:animate_easing(target, duration_sec, easing_val, anim_fn) end

--- Animates a component behind an ElementPointer.
---@param pointer ElementPointer Element pointer
---@param duration_sec number Animation duration in seconds
---@param easing Easing Easing curve
---@param anim_fn fun(comp: any, progress: number) Animation step callback
function Model:animate_pointer(pointer, duration_sec, easing, anim_fn) end

--- Mutates a component behind an ElementPointer directly.
---@param pointer_ud ElementPointer Element pointer
---@param func fun(comp: any) Callback function
function Model:with_pointer(pointer_ud, func) end


---=============================================================================
--- Typst Integration: Pager & LuaWorld
---=============================================================================

---@class Pager
---@field len integer Number of pages/snippets
---@field is_empty boolean True if pager has no content
Pager = {}

--- Creates a new Typst Pager container.
---@return Pager
function Pager.new() end

--- Adds a Typst markup/math string (chainable).
---@param content string Typst document markup (e.g., "$ E = m c^2 $")
---@return Pager
function Pager:add(content) end

--- Gets Typst code snippet at index (1-based).
---@param page integer Page index
---@return string|nil
function Pager:get(page) end

--- Returns number of pages/snippets.
---@return integer
function Pager:len() end

--- Checks if pager is empty.
---@return boolean
function Pager:is_empty() end


---@class LuaWorld
LuaWorld = {}

--- Creates a new Typst compiler world with a Pager.
---@param pager Pager Pager containing Typst snippets
---@return LuaWorld
function LuaWorld.new(pager) end

--- Overrides main source content with raw Typst string.
---@param src string Raw Typst markup string
function LuaWorld:raw_content(src) end

--- Compiles Typst document into an array of SvgBox elements.
---@return SvgBox[] Array of vector SvgBox elements
function LuaWorld:to_svgs() end
