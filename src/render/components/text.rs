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

impl mlua::FromLua for Text {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Text".to_string(),
            message: Some("expected Text UserData".to_string()),
        })
    }
}

#[derive(Clone, Copy)]
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

impl mlua::FromLua for Weight {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Weight".to_string(),
            message: Some("expected Weight UserData".to_string()),
        })
    }
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

#[derive(Clone, Copy)]
pub enum FontStyle {
    Regular,
    Italic,
    Oblique,
}

impl mlua::FromLua for FontStyle {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "FontStyle".to_string(),
            message: Some("expected FontStyle UserData".to_string()),
        })
    }
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

    pub fn set_color(&mut self, color: impl Into<String>) {
        self.color = Some(color.into());
        self.cache = None;
    }
    pub fn set_font(&mut self, font: impl Into<String>) {
        self.font = Some(font.into());
        self.cache = None;
    }
    pub fn set_size(&mut self, size: u32) {
        self.size = Some(size);
        self.cache = None;
    }
    pub fn set_weight(&mut self, weight: Weight) {
        self.weight = Some(weight);
        self.cache = None;
    }
    pub fn set_italic(&mut self) {
        self.style = Some(FontStyle::Italic);
        self.cache = None;
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

impl mlua::UserData for Weight {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("thin", |_, ()| Ok(Weight::Thin));
        methods.add_function("extra_light", |_, ()| Ok(Weight::ExtraLight));
        methods.add_function("light", |_, ()| Ok(Weight::Light));
        methods.add_function("regular", |_, ()| Ok(Weight::Reqular));
        methods.add_function("medium", |_, ()| Ok(Weight::Medium));
        methods.add_function("semi_bold", |_, ()| Ok(Weight::SemiBold));
        methods.add_function("bold", |_, ()| Ok(Weight::Bold));
        methods.add_function("extra_bold", |_, ()| Ok(Weight::ExtraBold));
        methods.add_function("black", |_, ()| Ok(Weight::Black));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("Weight::{}", this.as_typst()))
        });
    }
}

impl mlua::UserData for FontStyle {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("regular", |_, ()| Ok(FontStyle::Regular));
        methods.add_function("italic", |_, ()| Ok(FontStyle::Italic));
        methods.add_function("oblique", |_, ()| Ok(FontStyle::Oblique));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("FontStyle::{}", this.as_typst()))
        });
    }
}

impl mlua::UserData for Text {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("content", |_, this| Ok(this.content.clone()));
        fields.add_field_method_set("content", |_, this, val: String| {
            this.content = val;
            this.cache = None;
            Ok(())
        });
        fields.add_field_method_get("color", |_, this| Ok(this.color.clone()));
        fields.add_field_method_set("color", |_, this, val: Option<String>| {
            this.color = val;
            this.cache = None;
            Ok(())
        });
        fields.add_field_method_get("font", |_, this| Ok(this.font.clone()));
        fields.add_field_method_set("font", |_, this, val: Option<String>| {
            this.font = val;
            this.cache = None;
            Ok(())
        });
        fields.add_field_method_get("size", |_, this| Ok(this.size));
        fields.add_field_method_set("size", |_, this, val: Option<u32>| {
            this.size = val;
            this.cache = None;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, content: String| Ok(Text::new(content)));
        methods.add_method("color", |_, this, c: String| Ok(this.clone().color(c)));
        methods.add_method_mut("set_color", |_, this, c: String| {
            this.set_color(c);
            Ok(())
        });
        methods.add_method("font", |_, this, f: String| Ok(this.clone().font(f)));
        methods.add_method_mut("set_font", |_, this, f: String| {
            this.set_font(f);
            Ok(())
        });
        methods.add_method("size", |_, this, s: u32| Ok(this.clone().size(s)));
        methods.add_method_mut("set_size", |_, this, s: u32| {
            this.set_size(s);
            Ok(())
        });
        methods.add_method("weight", |_, this, w: Weight| Ok(this.clone().weight(w)));
        methods.add_method_mut("set_weight", |_, this, w: Weight| {
            this.set_weight(w);
            Ok(())
        });
        methods.add_method("italic", |_, this, ()| Ok(this.clone().italic()));
        methods.add_method_mut("set_italic", |_, this, ()| {
            this.set_italic();
            Ok(())
        });
        methods.add_method("to_typst", |_, this, ()| Ok(this.to_typst()));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(format!("Text(\"{}\")", this.content))
        });
    }
}
