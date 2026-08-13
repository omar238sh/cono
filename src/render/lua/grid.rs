use mlua::{AnyUserData, Function, UserData};

use crate::{
    down_cast,
    render::{
        components::{
            freebox::{FreeBox, Location},
            grid::Grid,
            style::{Alignment, Arrangement, Padding},
        },
        models::Size,
    },
};

impl mlua::FromLua for Grid {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Grid".to_string(),
            message: Some("expected Grid UserData".to_string()),
        })
    }
}

impl UserData for Grid {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("size", |_, this, ()| Ok(this.style.size()));
        methods.add_function("new", |_, (size, cols): (Size, Option<usize>)| {
            Ok(Grid::new(size, cols.unwrap_or(2)))
        });
        methods.add_function("from_size", |_, (w, h, cols): (f64, f64, Option<usize>)| {
            Ok(Grid::from_size(w, h, cols.unwrap_or(2)))
        });

        methods.add_method_mut("push", |_, this, val: mlua::AnyUserData| {
            down_cast!(val, |c| {
                this.push(c);
            });
            Ok(())
        });

        methods.add_method("cols", |_, this, cols: usize| Ok(this.clone().cols(cols)));
        methods.add_method_mut("set_cols", |_, this, cols: usize| {
            this.set_cols(cols);
            Ok(())
        });

        methods.add_method("rows", |_, this, rows: usize| Ok(this.clone().rows(rows)));
        methods.add_method_mut("set_rows", |_, this, rows: usize| {
            this.set_rows(rows);
            Ok(())
        });

        methods.add_method("gap", |_, this, gap: f64| Ok(this.clone().gap(gap)));
        methods.add_method_mut("set_gap", |_, this, gap: f64| {
            this.set_gap(gap);
            Ok(())
        });

        methods.add_method("col_gap", |_, this, gap: f64| Ok(this.clone().col_gap(gap)));
        methods.add_method_mut("set_col_gap", |_, this, gap: f64| {
            this.set_col_gap(gap);
            Ok(())
        });

        methods.add_method("row_gap", |_, this, gap: f64| Ok(this.clone().row_gap(gap)));
        methods.add_method_mut("set_row_gap", |_, this, gap: f64| {
            this.set_row_gap(gap);
            Ok(())
        });

        methods.add_method("arrangment", |_, this, arr: Arrangement| {
            Ok(this.clone().arrangment(arr))
        });
        methods.add_method_mut("set_arrangment", |_, this, arr: Arrangement| {
            this.set_arrangment(arr);
            Ok(())
        });
        methods.add_method("alignment", |_, this, align: Alignment| {
            Ok(this.clone().alignment(align))
        });
        methods.add_method_mut("set_alignment", |_, this, align: Alignment| {
            this.set_alignment(align);
            Ok(())
        });
        methods.add_method("padding", |_, this, pad: Padding| {
            Ok(this.clone().padding(pad))
        });
        methods.add_method_mut("set_padding", |_, this, pad: Padding| {
            this.set_padding(pad);
            Ok(())
        });

        methods.add_method_mut(
            "append_to_freebox",
            |_, this, (freebox, base_loc, callback): (AnyUserData, Location, Function)| {
                let mut fbox = freebox.borrow_mut::<FreeBox>()?;
                let call = |loc: Location| -> Location {
                    callback
                        .call::<Location>(loc)
                        .expect("Failed to execute Lua callback")
                };
                this.append_to_freebox(&mut *fbox, base_loc, call)
                    .map_err(|e| mlua::Error::external(e.to_string()))?;

                Ok(())
            },
        );
    }
}
