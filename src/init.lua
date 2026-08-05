-- local model = Model.new(Size.youtube_video(),FpsTarget.f120())

-- local text = Text.new("Hello");
-- text:set_size(15)
-- text:set_color("#853020")

-- local box = Container.new(text);
-- box:set_background("#755080")
-- box:set_border_radius(10)

-- local x = Coordinate.with_progress(
--   Coordinate.absolute(0),
--   Coordinate.absolute(0.5)
-- );
-- local y = Coordinate.with_progress(
--   Coordinate.absolute(0),
--   Coordinate.percentage(0.5)
-- )

-- local location = Location.new(x , y)

-- local ptr = model:spawn_located(box , location);

-- model:animate_pointer(ptr , 3 , Easing.linear() ,function (comp, progress)
--   comp:update_progress(progress)
-- end)

-- model:render_video("main.mp4");

-- Cono Lua: animated title card intro

local model = Model.new(Size.youtube_shorts(), FpsTarget.f60())

local background = Container.from_size(Size.youtube_shorts())
background:set_background("#FFFFFF")

model:add_component(background);

model:wait(1);

-- Build the card content
local title = Text.new("Hello, Cono"):size(10)
    title:set_size(56)
    title:set_color("#F5E0DC")
    title:set_weight(Weight.bold())

local subtitle = Text.new("programmatic video, made with Lua")
    subtitle:set_size(24)
    subtitle:set_color("#A6ADC8")

local col = Column.new(Size.new(900, 400))
    :arrangment(Arrangement.spaced_by(16))
    :alignment(Alignment.center())
    :padding(Padding.new(30.0))

col:push(title)
col:push(subtitle)

local card = Container.new(col)
    card:set_background("#1E1E2E")
    card:set_border_color("#89B4FA")
    card:set_border_width(5.0)
    card:set_border_radius(28.0)
    card:set_padding(Padding.new(40.0))

-- Entry motion: slide up from below center, fade in, rotate into place
local start_loc = Location.new(Coordinate.percentage(0.5), Coordinate.percentage(1.2))
local end_loc   = Location.new(Coordinate.percentage(0.5), Coordinate.percentage(0.5))
local loc = Location.with_progress(start_loc, end_loc)

local ptr = model:spawn_located(card, loc)

ptr:with_mut(
    function(c)
        c:set_anchor(Anchor.center())
        c:set_opacity(Opacity.with_progress(0.0, 1.0))
        c:set_angle(Angle.with_progress(-15.0, 0.0))

    end
)

-- Animate entrance over 1.2s, ease out
model:animate_pointer(ptr, 1.2, Easing.ease_out(), function(c, p)
    c:update_progress(p)
end)

-- Hold on screen
model:wait(2.5)

-- Exit: fade + slide up off-screen
model:with_pointer(ptr, function(c)
    local final_location = Location.new(Coordinate.percentage(0.5), Coordinate.absolute(-200))
    local new_loc = Location.with_progress(
        Location.new(Coordinate.percentage(0.5), Coordinate.percentage(0.5)),
        final_location
    )
    c:set_location(new_loc)
    c:set_anchor(Anchor.center())
    c:set_angle(Angle.new(0.0))
    c:set_opacity(Opacity.with_progress(1.0, 0.0))
end)

model:animate_pointer(ptr, 0.8, Easing.ease_in(), function(c, p)
    c:update_progress(p)
end)

model:render_video("intro.mp4")
print("Rendered intro.mp4")
