use std::path::Path;

use crate::{
    down_cast,
    render::{
        animation::{Easing, FpsTarget, Model},
        components::{
            freebox::Location,
            placeholder::{ElementPointer, PlaceHolder},
        },
        frame::Frame,
        models::Size,
        scene::Scene,
    },
};

impl mlua::FromLua for Easing {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Easing".to_string(),
            message: Some("expected Easing UserData".to_string()),
        })
    }
}

impl mlua::FromLua for Model {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Model".to_string(),
            message: Some("expected Model UserData".to_string()),
        })
    }
}

impl mlua::UserData for Easing {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("linear", |_, ()| Ok(Easing::Linear));
        methods.add_function("ease_in", |_, ()| Ok(Easing::EaseIn));
        methods.add_function("ease_out", |_, ()| Ok(Easing::EaseOut));
        methods.add_function("ease_in_out", |_, ()| Ok(Easing::EaseInOut));
        methods.add_method("ease", |_, this, t: f64| Ok(this.ease(t)));
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(match this {
                Easing::Linear => "Easing::Linear".to_string(),
                Easing::EaseIn => "Easing::EaseIn".to_string(),
                Easing::EaseOut => "Easing::EaseOut".to_string(),
                Easing::EaseInOut => "Easing::EaseInOut".to_string(),
                Easing::Custom(_) => "Easing::Custom(...)".to_string(),
            })
        });
    }
}

impl mlua::UserData for Model {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_, this| Ok(this.size()));
        fields.add_field_method_set("size", |_, this, size: Size| {
            this.set_size(size);
            Ok(())
        });
        fields.add_field_method_get("fps", |_, this| Ok(this.fps().value()));
        fields.add_field_method_set("fps", |_, this, fps: u32| {
            this.set_fps(FpsTarget::Custom(fps));
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, (size, fps_val): (Size, FpsTarget)| {
            Ok(Model::new(size, fps_val))
        });

        methods.add_method("size", |_lua, this, ()| Ok(this.size()));

        methods.add_method_mut("set_size", |_lua, this, size: Size| {
            this.set_size(size);
            Ok(())
        });

        methods.add_method("fps", |_lua, this, ()| Ok(this.fps().value()));

        methods.add_method_mut("set_fps", |_lua, this, fps_val: FpsTarget| {
            this.set_fps(fps_val);
            Ok(())
        });

        methods.add_method_mut("add_component", |_, this, val: mlua::AnyUserData| {
            down_cast!(this, add_component, val => [
                Text,
                Column,
                Row,
                Container,
                FreeBox,
                SvgBox,
                LocatedComponent,
                PlaceHolder<LocatedComponent>,
                PlaceHolder<Text>,
                PlaceHolder<Column>,
                PlaceHolder<Row>,
                PlaceHolder<Container>,
                PlaceHolder<FreeBox>,
                PlaceHolder<SvgBox>
            ]);
            Ok(())
        });

        methods.add_method_mut(
            "spawn_located",
            |_, this, (val, loc): (mlua::AnyUserData, Location)| {
                down_cast!(val, |c| {
                    this.spawn_located(c , loc.clone()).map_err(|e| {mlua::Error::external(e.to_string())})?
                }).ok_or_else(|| {mlua::Error::external(format!("Failed to downcast commponent"))})                
            },
        );

        methods.add_method("create_frame", |_, this, ()| Ok(this.create_frame()));

        methods.add_method_mut("push_frame", |_, this, frame: Frame| {
            this.push_frame(frame)
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_method("scene", |_, this, ()| Ok(this.scene().clone()));

        methods.add_method_mut("wait", |_lua, this, duration_sec: f64| {
            this.wait(duration_sec)
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_method("render_video", |_lua, this, path_str: String| {
            this.render_video(Path::new(&path_str))
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))
        });

        // Function-taking methods in Lua!
        methods.add_method_mut(
            "animate",
            |lua, this, (target, duration_sec, anim_fn): (mlua::AnyUserData, f64, mlua::Function)| {
                down_cast!(target, |c| {
                    let mut c = c;
                    this.animate(&mut c, duration_sec, |component , progress| {
                        anim_fn                    .call::<()>((
                        lua.create_userdata(component.clone())
                           .map_err(|e| mlua::Error::external(e)).unwrap(),
                        progress,
                    )).unwrap();
                    }).unwrap();   
                })
                    .ok_or_else(|| mlua::Error::UserDataTypeMismatch)?;
                
                
                Ok(())
            },
        );

        methods.add_method_mut(
            "animate_easing",
            |lua,
             this,
             (target, duration_sec, easing_val, anim_fn): (
                mlua::AnyUserData,
                f64,
                Easing,
                mlua::Function,
            )| {
            down_cast!(target, |c| {
                let mut c = c;
                this.animate_easing(&mut c, duration_sec, easing_val, |c_ref, progress| {
                        let _ = lua.scope(|scope| {
                            if let Ok(comp_ud) = scope.create_userdata_ref_mut(c_ref) {
                                anim_fn.call::<()>((
                                    comp_ud,
                                    progress,
                                ))?;
                            }
                            Ok(())
                        }).unwrap();
                    }).unwrap();
                } => [
                        Text,
                        Column,
                        Row,
                        Container,
                        FreeBox,
                        SvgBox,
                        LocatedComponent,
                        PlaceHolder<LocatedComponent>,
                        PlaceHolder<Text>,
                        PlaceHolder<Column>,
                        PlaceHolder<Row>,
                        PlaceHolder<Container>,
                        PlaceHolder<FreeBox>,
                        PlaceHolder<SvgBox>,
                        ElementPointer<Text>,
                        ElementPointer<Column>,
                        ElementPointer<Row>,
                        ElementPointer<Container>,
                        ElementPointer<FreeBox>,
                        ElementPointer<SvgBox>,
                        ElementPointer<LocatedComponent>,
                    ])
                .ok_or_else(|| mlua::Error::UserDataTypeMismatch)?;
                Ok(())  
            },
        );

        methods.add_method_mut(
            "animate_pointer",
            |lua,
             this,
             (pointer, duration_sec, easing, anim_fn): (
                mlua::AnyUserData,
                f64,
                Easing,
                mlua::Function,
            )| {
                down_cast!(pointer, |c| {
                    this.animate_pointer(c, duration_sec, easing, |c_ref, progress| {
                        lua.scope(|scope| {
                            let comp_ud = scope.create_userdata_ref_mut(c_ref).unwrap();
                            anim_fn.call::<()>((comp_ud, progress))?;
                            Ok(())
                        }).unwrap();
                    }).unwrap();
                } => [
                    ElementPointer<Text>,
                    ElementPointer<Column>,
                    ElementPointer<Row>,
                    ElementPointer<Container>,
                    ElementPointer<FreeBox>,
                    ElementPointer<SvgBox>,
                    ElementPointer<LocatedComponent>,
                ]);

                Ok(())
            },
        );
        
        

        methods.add_method(
            "with_pointer",
            |lua, this, (pointer_ud, func): (mlua::AnyUserData, mlua::Function)| {

                down_cast!(pointer_ud , |c| {
                    this.with_pointer(c, |c_ref| {
                        lua.scope(|scope| {

                            if let Ok(comp_ud) = scope.create_userdata_ref_mut(c_ref) {
                                func.call::<()>(comp_ud)?;
                            }

                            Ok(())
                        }).unwrap();
                    })
                } => [
                        ElementPointer<Text>,
                        ElementPointer<Column>,
                        ElementPointer<Row>,
                        ElementPointer<Container>,
                        ElementPointer<FreeBox>,
                        ElementPointer<SvgBox>,
                        ElementPointer<LocatedComponent>
                ]);
                Ok(())
            },
        );

        methods.add_meta_method(mlua::MetaMethod::Mul, |_, this, other_val: mlua::Value| {
            if let mlua::Value::UserData(ud) = &other_val {
                if let Ok(other_model) = ud.borrow::<Model>() {
                    return Ok(this.clone() * other_model.clone());
                }
                if let Ok(other_scene) = ud.borrow::<Scene>() {
                    return Ok(this.clone() * other_scene.clone());
                }
            }
            Err(mlua::Error::external(
                "Invalid operand for Model multiplication",
            ))
        });
    }
}
