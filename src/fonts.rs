use std::ops::Deref;

use typst::text::Font;

use crate::assets::Assets;

pub struct Fonts {
    pub fonts: Vec<Font>,
}

impl Fonts {
    pub fn new() -> Self {
        Self { fonts: Vec::new() }
    }

    pub fn add(mut self, font_name: &str) -> Self {
        self.fonts.push(Assets::get_font(font_name).unwrap());
        self
    }

    pub fn add_ttf(mut self, font_name: &str) -> Self {
        self.fonts.push(Assets::get_ttf(font_name).unwrap());
        self
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Fonts::new()
            .add("NewCM08-Book")
            .add("NewCM08-BookItalic")
            .add("NewCM08-Italic")
            .add("NewCM08-Regular")
            .add("NewCM08Devanagari-Book")
            .add("NewCM08Devanagari-Regular")
            .add("NewCM10-Bold")
            .add("NewCM10-BoldItalic")
            .add("NewCM10-Book")
            .add("NewCM10-BookItalic")
            .add("NewCM10-Italic")
            .add("NewCM10-Regular")
            .add("NewCM10Devanagari-Bold")
            .add("NewCM10Devanagari-Book")
            .add("NewCM10Devanagari-Regular")
            .add("NewCMMath-Bold")
            .add("NewCMMath-Book")
            .add("NewCMMath-Regular")
            .add("NewCMMono10-Bold")
            .add("NewCMMono10-BoldOblique")
            .add("NewCMMono10-Book")
            .add("NewCMMono10-BookItalic")
            .add("NewCMMono10-Italic")
            .add("NewCMMono10-Regular")
            .add("NewCMSans08-Book")
            .add("NewCMSans08-BookOblique")
            .add("NewCMSans08-Oblique")
            .add("NewCMSans08-Regular")
            .add("NewCMSans10-Bold")
            .add("NewCMSans10-BoldOblique")
            .add("NewCMSans10-Book")
            .add("NewCMSans10-BookOblique")
            .add("NewCMSans10-Oblique")
            .add("NewCMSans10-Regular")
            .add("NewCMSansMath-Regular")
            .add("NewCMUncial08-Bold")
            .add("NewCMUncial08-Book")
            .add("NewCMUncial08-Regular")
            .add("NewCMUncial10-Bold")
            .add("NewCMUncial10-Book")
            .add("NewCMUncial10-Regular")
            .add_ttf("NotoKufiArabic-Black")
            .add_ttf("NotoKufiArabic-Bold")
            .add_ttf("NotoKufiArabic-ExtraBold")
            .add_ttf("NotoKufiArabic-ExtraLight")
            .add_ttf("NotoKufiArabic-Light")
            .add_ttf("NotoKufiArabic-Medium")
            .add_ttf("NotoKufiArabic-Regular")
            .add_ttf("NotoKufiArabic-SemiBold")
            .add_ttf("NotoKufiArabic-Thin")
            .add_ttf("NotoKufiArabic-VariableFont_wght")
    }
}

impl Deref for Fonts {
    type Target = Vec<Font>;

    fn deref(&self) -> &Self::Target {
        &self.fonts
    }
}
