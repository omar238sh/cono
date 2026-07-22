use std::{
    fmt::Write as _,
    sync::{LazyLock, Mutex},
};
use taffy::NodeId;

use crate::render::{
    components::svg::SvgBox,
    error::RenderError,
    frame::Component,
    models::Size,
    typst::{LuaWorld, pager::Pager},
};

static COMPILER: LazyLock<Mutex<LuaWorld>> =
    LazyLock::new(|| Mutex::new(LuaWorld::new(Pager::new())));

#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub color: Option<String>,
    pub font: Option<String>,
    pub size: Option<u32>,
    pub weight: Option<Weight>,
    pub style: Option<FontStyle>,
    cache: Option<SvgBox>,
    pub id: Option<NodeId>,
}

#[derive(Clone)]
pub enum Weight {
    Thin,
    ExtraLight,
    Light,
    Reqular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl Weight {
    pub fn as_typst(&self) -> &'static str {
        match self {
            Weight::Thin => "thin",
            Weight::ExtraLight => "extralight",
            Weight::Light => "light",
            Weight::Reqular => "regular",
            Weight::Medium => "medium",
            Weight::SemiBold => "semibold",
            Weight::Bold => "bold",
            Weight::ExtraBold => "extrabold",
            Weight::Black => "black",
        }
    }
}

#[derive(Clone)]
pub enum FontStyle {
    Regular,
    Italic,
    Oblique,
}

impl FontStyle {
    pub fn as_typst(&self) -> &'static str {
        match self {
            FontStyle::Regular => "normal",
            FontStyle::Italic => "italic",
            FontStyle::Oblique => "oblique",
        }
    }
}

impl Text {
    pub fn new(content: impl ToString) -> Self {
        Self {
            content: content.to_string(),
            color: None,
            font: None,
            size: None,
            weight: None,
            style: None,
            cache: None,
            id: None,
        }
    }

    pub fn ensure_compiled(&self) -> Result<(), RenderError> {
        match self.cache {
            Some(_) => Ok(()),
            None => Err(RenderError::CustomError(
                "Text hasn't be compiled!".to_string(),
            )),
        }
    }

    pub fn to_typst(&self) -> String {
        let mut args: Vec<String> = Vec::new();

        if let Some(color) = &self.color {
            args.push(format!("fill: {}", Self::color_to_typst(color)));
        }
        if let Some(font) = &self.font {
            args.push(format!("font: \"{}\"", Self::escape_string(font)));
        }
        if let Some(size) = &self.size {
            args.push(format!("size: {}pt", size));
        }
        if let Some(weight) = &self.weight {
            args.push(format!("weight: \"{}\"", weight.as_typst()));
        }
        if let Some(style) = &self.style {
            args.push(format!("style: \"{}\"", style.as_typst()));
        }

        let body = Self::escape_markup(&self.content);

        let mut out = String::new();
        if args.is_empty() {
            // No styling needed — just emit the raw markup content.
            out.push_str(&body);
        } else {
            let _ = write!(
                out,
                r#"
                #set page(
                    width : auto,
                    height : auto,
                    fill : none,
                    margin : 0.4cm
                )
                #text({})[{}]"#,
                args.join(", "),
                body
            );
        }

        out
    }

    fn color_to_typst(color: &str) -> String {
        let trimmed = color.trim();
        if trimmed.starts_with('#') {
            format!("rgb(\"{}\")", trimmed)
        } else if trimmed.chars().all(|c| c.is_ascii_hexdigit()) && trimmed.len() == 6 {
            format!("rgb(\"#{}\")", trimmed)
        } else {
            // Assume it's a named Typst color/identifier (e.g. "red", "blue").
            trimmed.to_string()
        }
    }

    fn escape_markup(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '\\' | '#' | '*' | '_' | '$' | '[' | ']' | '<' | '>' | '@' | '`' => {
                    out.push('\\');
                    out.push(c);
                }
                _ => out.push(c),
            }
        }
        out
    }

    fn escape_string(s: &str) -> String {
        s.replace('\\', "\\\\").replace('"', "\\\"")
    }

    fn sanitize_label(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' || c == ':' {
                    c
                } else {
                    '-'
                }
            })
            .collect()
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font = Some(font.into());
        self
    }
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }
    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = Some(weight);
        self
    }
    pub fn italic(mut self) -> Self {
        self.style = Some(FontStyle::Italic);
        self
    }

    fn compile(&mut self) -> Result<(), RenderError> {
        let mut world = COMPILER
            .lock()
            .map_err(|_| RenderError::TypstCompilerError)?;
        let src = self.to_typst();
        world.raw_content(src);
        let svg = world.to_svgs()?.pop();
        self.cache = svg;
        Ok(())
    }
}

impl Component for Text {
    fn render(
        &self,
        tree: &taffy::TaffyTree,
    ) -> Result<vello::Scene, crate::render::error::RenderError> {
        match &self.cache {
            Some(v) => v.render(tree),
            None => Err(RenderError::CustomError(
                "Text hasn't been compiled!".to_string(),
            )),
        }
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        match &self.cache {
            Some(v) => v.size(),
            None => Err(RenderError::NotComputed), // won't happened
        }
    }

    fn register_layout(
        &mut self,
        tree: &mut taffy::TaffyTree,
    ) -> std::result::Result<taffy::NodeId, RenderError> {
        self.compile()?;
        self.cache.as_mut().unwrap().register_layout(tree)
    }

    fn layout(&self) -> Result<taffy::NodeId, crate::render::error::RenderError> {
        match &self.cache {
            Some(v) => v.layout(),
            None => Err(RenderError::NotComputed),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
