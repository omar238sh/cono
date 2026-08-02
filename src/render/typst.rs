use chrono::{Datelike, Local, Timelike};
use std::ops::Deref;
use typst::{
    Library, LibraryExt, World,
    diag::{FileResult, SourceResult, Warned},
    foundations::{Bytes, Datetime},
    layout::PagedDocument,
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_svg::svg;

use crate::{
    fonts::Fonts,
    render::{components::svg::SvgBox, error::RenderError, typst::pager::Pager},
};

pub mod pager {
    #![allow(unused)]

    use std::fmt::Display;
    #[deprecated]
    #[derive(Clone)]
    pub struct Pager {
        content: Vec<String>,
    }

    impl mlua::FromLua for Pager {
        fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
            if let mlua::Value::UserData(ud) = value {
                return Ok(ud.borrow::<Self>()?.clone());
            }
            Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Pager".to_string(),
                message: Some("expected Pager UserData".to_string()),
            })
        }
    }

    impl Pager {
        pub fn new() -> Self {
            Self {
                content: Vec::new(),
            }
        }

        pub fn add(mut self, content: impl Display) -> Self {
            self.content.push(format!(
                r#"
                #set page(
                    width : auto,
                    height : auto,
                    fill : none,
                    margin : 0.4cm
                )
                {content}
                "#
            ));
            self
        }

        pub fn get(&self, page: usize) -> Option<&String> {
            self.content.get(page)
        }

        pub fn len(&self) -> usize {
            self.content.len()
        }

        pub fn is_empty(&self) -> bool {
            self.content.is_empty()
        }
    }

    impl Display for Pager {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.content.join("\n#pagebreak()\n"))
        }
    }

    impl mlua::UserData for Pager {
        fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
            fields.add_field_method_get("len", |_, this| Ok(this.len()));
            fields.add_field_method_get("is_empty", |_, this| Ok(this.is_empty()));
        }

        fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
            methods.add_function("new", |_, ()| Ok(Pager::new()));
            methods.add_method("add", |_, this, content: String| Ok(this.clone().add(content)));
            methods.add_method("get", |_, this, page: usize| Ok(this.get(page).cloned()));
            methods.add_method("len", |_, this, ()| Ok(this.len()));
            methods.add_method("is_empty", |_, this, ()| Ok(this.is_empty()));
        }
    }
}

impl mlua::UserData for LuaWorld {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, pager: pager::Pager| Ok(LuaWorld::new(pager)));
        methods.add_method_mut("raw_content", |_, this, src: String| {
            this.raw_content(src);
            Ok(())
        });
        methods.add_method("to_svgs", |_, this, ()| {
            this.to_svgs().map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });
    }
}

pub struct LuaWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Fonts,
    main_id: FileId,
    main_source: Source,
    creation_time: Option<Datetime>,
}

impl LuaWorld {
    pub fn new(content: Pager) -> Self {
        let library = Library::builder().build();
        let fonts = Fonts::default();

        let book = FontBook::from_fonts(fonts.deref());
        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(main_id, content.to_string());
        let now = Local::now();

        let creation_time = Datetime::from_ymd_hms(
            now.year(),
            now.month() as u8,
            now.day() as u8,
            now.hour() as u8,
            now.minute() as u8,
            now.second() as u8,
        );
        Self {
            library: LazyHash::new(library),
            book: LazyHash::new(book),
            fonts,
            main_id,
            main_source: source,
            creation_time,
        }
    }

    pub fn raw_content(&mut self, src: impl ToString) {
        self.main_source = Source::new(self.main_id, src.to_string());
    }

    pub fn to_svgs(&self) -> Result<Vec<SvgBox>, RenderError> {
        let document = match typst::compile::<PagedDocument>(self) {
            Warned {
                output: Ok(doc), ..
            } => doc,
            Warned {
                output: Err(errors),
                ..
            } => {
                return Err(RenderError::TypstError(format!("{errors:?}")));
            }
        };
        let mut results = Vec::new();
        if document.pages.len() >= 1 {
            for page in &document.pages {
                results.push(SvgBox::new(svg(page))?);
            }
            return Ok(results);
        } else {
            return Ok(Vec::new());
        }
    }
}

impl World for LuaWorld {
    #[doc = " The standard library."]
    #[doc = ""]
    #[doc = " Can be created through `Library::build()`."]
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    #[doc = " Metadata about all known fonts."]
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    #[doc = " Get the file id of the main source file."]
    fn main(&self) -> FileId {
        self.main_id
    }

    #[doc = " Try to access the specified source file."]
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            Ok(self.main_source.clone())
        } else {
            Err(typst::diag::FileError::AccessDenied)
        }
    }

    #[doc = " Try to access the specified file."]
    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(typst::diag::FileError::AccessDenied)
    }

    #[doc = " Try to access the font with the given index in the font book."]
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    #[doc = " Get the current date."]
    #[doc = ""]
    #[doc = " If no offset is specified, the local date should be chosen. Otherwise,"]
    #[doc = " the UTC date should be chosen with the corresponding offset in hours."]
    #[doc = ""]
    #[doc = " If this function returns `None`, Typst\'s `datetime` function will"]
    #[doc = " return an error."]
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        self.creation_time
    }
}
