use vello::peniko::color::LinearSrgb;

use crate::render::typst::{LuaWorld, pager::Pager};

mod assets;
mod fonts;
mod render;
fn main() {
    let world = LuaWorld::new(
        Pager::new()
            .add("$ F(omega) = integral_(-oo)^(+oo) f(t) dot e^(-i omega t) d t $")
            .add("$x = x + 1$")
            .add("$y = (x + 1) / 3$"),
    );
}
