use mlua::{AnyUserData, Function};

use crate::{
    down_cast,
    render::{
        components::freebox::{
            Anchor, Angle, Coordinate, FreeBox, LocatedComponent, Location, Opacity,
        },
        models::Size,
    },
};

impl mlua::FromLua for FreeBox {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "FreeBox".to_string(),
            message: Some("expected FreeBox UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Coordinate {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Coordinate".to_string(),
            message: Some("expected Coordinate UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Anchor {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Anchor".to_string(),
            message: Some("expected Anchor UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Location {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Location".to_string(),
            message: Some("expected Location UserData".to_string()),
        })
    }
}

impl mlua::FromLua for LocatedComponent {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "LocatedComponent".to_string(),
            message: Some("expected LocatedComponent UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Opacity {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Opacity".to_string(),
            message: Some("expected Opacity UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Angle {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Angle".to_string(),
            message: Some("expected Angle UserData".to_string()),
        })
    }
}

impl mlua::UserData for Opacity {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, v: f64| Ok(Opacity::new(v)));
        methods.add_function("with_progress", |_, (start, end): (f64, f64)| {
            Ok(Opacity::with_progress(start, end))
        });
        methods.add_method_mut("update_progress", |_, this, p: f64| {
            this.update_progress(p);
            Ok(())
        });
        methods.add_method("value", |_, this, ()| Ok(this.value()));
    }
}

impl mlua::UserData for Angle {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, v: f64| Ok(Angle::new(v)));
        methods.add_function("with_progress", |_, (start, end): (f64, f64)| {
            Ok(Angle::with_progress(start, end))
        });
        methods.add_method_mut("update_progress", |_, this, p: f64| {
            this.update_progress(p);
            Ok(())
        });
        methods.add_method("value", |_, this, ()| Ok(this.value()));
    }
}

impl mlua::UserData for Coordinate {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("absolute", |_, v: f64| Ok(Coordinate::absolute(v)));
        methods.add_function("percentage", |_, v: f64| Ok(Coordinate::percentage(v)));
        methods.add_function("zero", |_, ()| Ok(Coordinate::zero()));
        methods.add_function(
            "with_progress",
            |_, (start, end): (Coordinate, Coordinate)| Ok(Coordinate::with_progress(start, end)),
        );
        methods.add_method("scale", |_, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
        methods.add_method("to_absolute", |_, this, max_len: f64| {
            Ok(this.to_absolute(max_len))
        });
        methods.add_method("to_num", |_, this, max_len: f64| Ok(this.to_num(max_len)));
        methods.add_method_mut("update_progress", |_, this, p: f64| {
            this.update_progress(p);
            Ok(())
        });

        methods.add_meta_method(mlua::MetaMethod::Add, |_, this, other: Coordinate| {
            Ok(this.clone() + other)
        });
        methods.add_meta_method(mlua::MetaMethod::Sub, |_, this, other: Coordinate| {
            Ok(this.clone() - other)
        });
        methods.add_meta_method(mlua::MetaMethod::Mul, |_, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(match this {
                Coordinate::Absolute(v) => format!("Coordinate::Absolute({})", v),
                Coordinate::Percentage(v) => format!("Coordinate::Percentage({})", v),
                _ => "Coordinate(...)".to_string(),
            })
        });
    }
}

impl mlua::UserData for Anchor {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("center", |_, ()| Ok(Anchor::Center));
        methods.add_function("top_left", |_, ()| Ok(Anchor::TopLeft));
        methods.add_function("top_right", |_, ()| Ok(Anchor::TopRight));
        methods.add_function("bottom_left", |_, ()| Ok(Anchor::BottomLeft));
        methods.add_function("bottom_right", |_, ()| Ok(Anchor::BottomRight));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(match this {
                Anchor::Center => "Anchor::Center".to_string(),
                Anchor::TopLeft => "Anchor::TopLeft".to_string(),
                Anchor::TopRight => "Anchor::TopRight".to_string(),
                Anchor::BottomLeft => "Anchor::BottomLeft".to_string(),
                Anchor::BottomRight => "Anchor::BottomRight".to_string(),
            })
        });
    }
}

impl mlua::UserData for Location {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x.clone()));
        fields.add_field_method_set("x", |_, this, coord: Coordinate| {
            this.x = coord;
            Ok(())
        });
        fields.add_field_method_get("y", |_, this| Ok(this.y.clone()));
        fields.add_field_method_set("y", |_, this, coord: Coordinate| {
            this.y = coord;
            Ok(())
        });
        fields.add_field_method_get("anchor", |_, this| Ok(this.anchor.clone()));
        fields.add_field_method_set("anchor", |_, this, anchor: Anchor| {
            this.anchor = anchor;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, (x, y): (Coordinate, Coordinate)| {
            Ok(Location::new(x, y))
        });
        methods.add_function("zero", |_, ()| Ok(Location::zero()));
        methods.add_function("center", |_, anchor: Anchor| Ok(Location::center(anchor)));
        methods.add_function("with_progress", |_, (loc1, loc2): (Location, Location)| {
            Ok(Location::with_progress(loc1, loc2))
        });

        methods.add_method("scale", |_, this, factor: f64| {
            Ok(this.clone().scale(factor))
        });
        methods.add_method(
            "as_topleft",
            |_, this, size: Size| Ok(this.as_topleft(size)),
        );
        methods.add_method_mut("update_progress", |_, this, p: f64| {
            this.update_progress(p);
            Ok(())
        });

        methods.add_meta_method(mlua::MetaMethod::Add, |_, this, other: Location| {
            Ok(this.clone() + other)
        });
        methods.add_meta_method(mlua::MetaMethod::Sub, |_, this, other: Location| {
            Ok(this.clone() - other)
        });
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, _, ()| {
            Ok("Location(...)".to_string())
        });
    }
}

impl mlua::UserData for LocatedComponent {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("location", |_, this| Ok(this.get_location()));
        fields.add_field_method_set("location", |_, this, loc: Location| {
            this.set_location(loc);
            Ok(())
        });
        fields.add_field_method_get("opacity", |_, this| Ok(this.get_opacity()));
        fields.add_field_method_set("opacity", |_, this, op: Opacity| {
            this.opacity = op;
            Ok(())
        });
        fields.add_field_method_get("angle", |_, this| Ok(this.get_angle()));
        fields.add_field_method_set("angle", |_, this, ang: Angle| {
            this.angle = ang;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, val: mlua::AnyUserData| {
            let located = down_cast!(val, |c| { LocatedComponent::new(c) }).unwrap();
            Ok(located)
        });

        methods.add_method_mut("set_component", |_, this, comp: AnyUserData| {
            down_cast!(comp, |c| {
                this.set_component(c);
            })
            .ok_or(mlua::Error::external("mismatched type"))?;
            Ok(())
        });

        methods.add_method_mut("update_progress", |_, this, progress| {
            this.update_progress(progress);
            Ok(())
        });

        methods.add_method("location", |_, this, loc: Location| {
            Ok(this.clone().location(loc))
        });
        methods.add_method_mut("set_location", |_, this, loc: Location| {
            this.set_location(loc);
            Ok(())
        });

        methods.add_method_mut("target_loc", |_, this, (x, y): (Coordinate, Coordinate)| {
            let loc = this.get_location();
            let new_loc = Location::with_progress(loc, Location::new(x, y));
            this.set_location(new_loc);
            Ok(())
        });

        methods.add_method("anchor", |_, this, anchor: Anchor| {
            Ok(this.clone().anchor(anchor))
        });
        methods.add_method_mut("set_anchor", |_, this, anchor: Anchor| {
            this.set_anchor(anchor);
            Ok(())
        });
        methods.add_method("opacity", |_, this, op: Opacity| {
            let mut c = this.clone();
            c.opacity = op;
            Ok(c)
        });
        methods.add_method_mut("set_opacity", |_, this, op: Opacity| {
            this.opacity = op;
            Ok(())
        });

        methods.add_method_mut("target_opacity", |_, this, opacity: Opacity| {
            let op = this.get_opacity();
            let new_op = Opacity::with_progress(op.value(), opacity.value());
            this.opacity = new_op;
            Ok(())
        });

        methods.add_method_mut("pin", |_, this, ()| {
            let mut loc = this.get_location();
            loc.x.pin();
            loc.y.pin();
            this.set_location(loc);

            let opacity = this.get_opacity();
            this.opacity = Opacity::new(opacity.value());

            let angle = this.get_angle();
            this.angle = Angle::new(angle.value());

            Ok(())
        });

        methods.add_method_mut("pin_inner", |_, this, ()| {
            this.pin();
            Ok(())
        });

        methods.add_method_mut("unpin_inner", |_, this, ()| {
            this.unpin();
            Ok(())
        });

        methods.add_method("angle", |_, this, ang: Angle| {
            let mut c = this.clone();
            c.angle = ang;
            Ok(c)
        });
        methods.add_method_mut("set_angle", |_, this, ang: Angle| {
            this.angle = ang;
            Ok(())
        });

        methods.add_method_mut("target_angle", |_, this, angle: Angle| {
            let a = this.get_angle();
            let new_angle = Angle::with_progress(a.value(), angle.value());
            this.angle = new_angle;
            Ok(())
        });
        methods.add_method_mut("shift", |_, this, shift_loc: Location| {
            this.shift(shift_loc);
            Ok(())
        });
        methods.add_method_mut("sync_location", |_, this, ()| {
            this.sync_location();
            Ok(())
        });
        methods.add_method("get_location", |_, this, ()| Ok(this.get_location()));
    }
}

impl mlua::UserData for FreeBox {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.size));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, size: Size| Ok(FreeBox::new(size)));
        methods.add_method("with_child", |_, this, child: LocatedComponent| {
            Ok(this.clone().with_child(child))
        });
        methods.add_method_mut("add", |_, this, val: mlua::AnyUserData| {
            down_cast!(val, |a| {
                this.add(a.clone());
            });
            Ok(())
        });

        methods.add_method_mut("for_each", |lua, this, f: Function| {
            let func = |located_comp: &mut LocatedComponent| {
                lua.scope(|scope| {
                    let a = scope.create_userdata_ref_mut(located_comp)?;
                    f.call::<LocatedComponent>(a)?;
                    Ok(())
                })?;
                Ok::<(), mlua::Error>(())
            };
            this.contents
                .iter_mut()
                .for_each(|item| func(item).expect("failed to run callback"));
            Ok(())
        });
    }
}
