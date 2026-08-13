local compose = require("compose")

local scene = compose.scene(Size.youtube_shorts(), FpsTarget.f60())

local title = compose.text("Golden Ratio", 56, "#F5E0DC", Weight.bold())
local eq = compose.math("phi = (1 + sqrt(5)) / 2", 48)

scene
    :background()
    :actor("t", title, compose.center())
    :actor("e", eq, compose.center())

    :enter("t", compose.fade_in(1.0))
    :hold(2.5)
    :exit("t", compose.fade_out(0.8))

    :enter("e", compose.spin_in(1.2))
    :hold(3.0)
    :pulse("e", compose.pulse(0.4, 5.0))
    :exit("e", compose.slide_out("down", 0.8))

scene:render("output.mp4")
