local model = Model.new(Size.youtube_video(),FpsTarget.f120())

local text = Text.new("Hello");
text:set_size(15)
text:set_color("#853020")

local box = Container.new(text);
box:set_background("#755080")
box:set_border_radius(10)

local x = Coordinate.with_progress(
  Coordinate.absolute(0),
  Coordinate.absolute(0.5)
);
local y = Coordinate.with_progress(
  Coordinate.absolute(0),
  Coordinate.percentage(0.5)
)

local location = Location.new(x , y)

local ptr = model:spawn_located(box , location);

model:animate_pointer(ptr , 3 , Easing.linear() ,function (comp, progress)
  comp:update_progress(progress)
end)

model:render_video("main.mp4");
