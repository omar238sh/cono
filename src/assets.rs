use rust_embed::Embed;
use typst::{foundations::Bytes, text::Font};

#[derive(Embed)]
#[folder = "assets/"]
pub struct Assets;

impl Assets {
    pub fn get_font(font_name: &str) -> Option<Font> {
        let font = Self::get(&format!("fonts/otf/{font_name}.otf"))?;
        let bytes = Bytes::new(font.data);
        Font::new(bytes, 0)
    }
}
