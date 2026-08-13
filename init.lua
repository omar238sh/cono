local compose = require("compose")
local icons = require("icons")
local T = compose.themes.sunset

local scene = compose.scene(Size.youtube_shorts(), FpsTarget.f60())
scene:background(T.bg)

local logo_icon = icons.load("logo", 0.5, 12)
local math_icon = icons.load("sigma", 1.8, 8)
local code_icon = icons.load("code", 1.8, 8)
local check_icon = icons.inline("check", 28)

local title = compose.text("Cono Engine", 56, T.text, Weight.bold())
local subtitle = compose.text("Programmatic Video", 28, T.text_secondary)

local header = compose.stack_vertical({title, subtitle}, Size.new(900, 200), 12)

local card1 = compose.card(
    compose.stack_vertical({math_icon, compose.text("Math", 24, T.accent)}, Size.new(200, 180), 16),
    T.surface, T.accent, 24, 20
)

local card2 = compose.card(
    compose.stack_vertical({code_icon, compose.text("Code", 24, T.accent)}, Size.new(200, 180), 16),
    T.surface, T.accent, 24, 20
)

local cards = compose.stack_horizontal({card1, card2}, Size.new(900, 220), 24)

local footer = compose.text("Built with Lua + Rust", 22, T.text_secondary)

scene
    :actor("logo", logo_icon, compose.top(0.12))
    :actor("header", header, compose.center(0.32))
    :actor("cards", cards, compose.center(0.58))
    :actor("footer", footer, compose.bottom(0.88))

    :enter("logo", compose.bounce_in(1.0))
    :enter("header", compose.fade_in(0.8):delay(0.3))
    :stagger({"cards"}, compose.slide_in("up", 0.8), 0.0)
    :enter("footer", compose.fade_in(0.6):delay(0.6))
    :hold(2.5)
    :pulse("logo", compose.pulse(0.4, 8.0))
    :hold(1.0)
    :group_exit({"logo", "header", "cards", "footer"}, compose.fade_out(0.8))

scene:render("icons_demo.mp4")
print("Rendered: icons_demo.mp4")
