use resvg::render;
use tiny_skia::{Paint, PathBuilder, Pixmap};
use usvg::{Options, Transform};

use crate::render::{error::RenderError, frame::Component, models::Size};

struct SvgBox {
    content: Pixmap,
}

impl Component for SvgBox {
    fn render(&self) -> &Pixmap {
        &self.content
    }

    fn content_mut(&mut self) -> &mut Pixmap {
        &mut self.content
    }
}

impl SvgBox {
    pub fn new(content: String) -> Result<Self, RenderError> {
        let tree = usvg::Tree::from_str(&content, &Options::default())?;
        let mut pixmap = Pixmap::new(tree.size().width() as u32, tree.size().height() as u32)
            .ok_or(RenderError::CannotCreatePixmap)?;

        render(&tree, Transform::identity(), &mut pixmap.as_mut());

        Ok(Self { content: pixmap })
    }
}

#[cfg(test)]
mod cvg_test {
    use crate::render::{
        components::svg::SvgBox,
        error::RenderError,
        frame::Component,
        typst::{LuaWorld, pager::Pager},
    };

    #[test]
    fn test_svg() -> Result<(), RenderError> {
        let content = LuaWorld::new(Pager::new().add("مرحبا"))
            .to_svgs()
            .map_err(|_| RenderError::CustomError("cannot create svgs".to_string()))?;
        for svg in content {
            let mysvg_box = SvgBox::new(svg)?;
            let p = mysvg_box.render();
            p.save_png("./tests/svg.png")
                .map_err(|a| RenderError::CustomError(format!("Cannot Save png {a}")))?;
        }
        Ok(())
    }
}
