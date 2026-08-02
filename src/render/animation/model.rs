use std::{ops::Mul, path::Path};

use crate::{
    down_cast,
    render::{
        animation::FpsTarget,
        components::{
            column::Column,
            freebox::{Coordinate, LocatedComponent, Location},
            placeholder::{ElementPointer, PlaceHolder},
            row::Row,
            text::Text,
        },
        error::RenderError,
        frame::{Component, Frame},
        models::Size,
        scene::{Scene, multiply_scenes},
    },
};

/// Easing functions for timing-controlled animations.
#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Custom(fn(f64) -> f64),
}

impl Easing {
    pub fn ease(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => t * (2.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::Custom(f) => f(t),
        }
    }
}

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

/// `Model` offers high-level animation, composition, timing, and frame synchronization control,
/// while maintaining low-level access to components and underlying `Scene` structures.
///
/// Designed with full compatibility for Lua bindings (via `mlua::UserData`).
#[derive(Clone)]
pub struct Model {
    size: Size,
    scene: Scene,
    components: Vec<LocatedComponent>,
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

impl Model {
    /// Creates a new `Model` with a specified canvas `Size` and target FPS.
    pub fn new(size: Size, fps: FpsTarget) -> Self {
        let mut scene = Scene::new();
        scene.set_fps(fps);
        Self {
            size,
            scene,
            components: Vec::new(),
        }
    }

    /// Returns the target canvas size.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Sets the target canvas size.
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Returns the configured target FPS.
    pub fn fps(&self) -> FpsTarget {
        self.scene.fps()
    }

    /// Sets the target FPS for timing calculation and frame synchronization.
    pub fn set_fps(&mut self, fps: FpsTarget) {
        self.scene.set_fps(fps);
    }

    // =========================================================================
    // Ergonomic Component & Pointer/Placeholder Management (Minimal Variables)
    // =========================================================================

    /// Spawns a component directly into the model inside a `PlaceHolder` and returns its `ElementPointer`.
    /// Reduces variable boilerplate down to a single line.
    pub fn spawn<C: Component + Clone + 'static>(
        &mut self,
        component: C,
    ) -> Result<ElementPointer<C>, RenderError> {
        let placeholder = PlaceHolder::from(component)?;
        let pointer = placeholder.pointer();
        self.components.push(placeholder.locate());
        Ok(pointer)
    }

    /// Spawns a component with a specific `Location` directly into the model inside a `PlaceHolder`
    /// and returns an `ElementPointer` to the `LocatedComponent`.
    /// Reduces variable boilerplate down to a single line.
    pub fn spawn_located<C: Component + Clone + 'static>(
        &mut self,
        component: C,
        location: Location,
    ) -> Result<ElementPointer<LocatedComponent>, RenderError> {
        let located = LocatedComponent::new(component).location(location);
        let placeholder = PlaceHolder::from(located)?;
        let pointer = placeholder.pointer();
        self.components.push(placeholder.locate());
        Ok(pointer)
    }

    /// Registers an existing `PlaceHolder` component into the model and returns its `ElementPointer`.
    pub fn add_placeholder<C: Component + Clone + 'static>(
        &mut self,
        placeholder: PlaceHolder<C>,
    ) -> ElementPointer<C> {
        let pointer = placeholder.pointer();
        self.components.push(placeholder.locate());
        pointer
    }

    /// Helper to construct a `(PlaceHolder<C>, ElementPointer<C>)` tuple without immediately adding it to `Model`.
    /// Useful for nested layouts (e.g. Columns, Rows) while still avoiding multiple temporary variables.
    pub fn create_pointer<C: Component + Clone + 'static>(
        &self,
        component: C,
    ) -> Result<(PlaceHolder<C>, ElementPointer<C>), RenderError> {
        let placeholder = PlaceHolder::from(component)?;
        let pointer = placeholder.pointer();
        Ok((placeholder, pointer))
    }

    /// Registers a raw component into the model's base scene composition.
    pub fn add_component<C: Component + 'static>(&mut self, component: C) -> &mut Self {
        self.components.push(component.locate());
        self
    }

    // =========================================================================
    // Low-Level Accessors & Operations
    // =========================================================================

    /// Access low-level registered base components.
    pub fn components(&self) -> &[LocatedComponent] {
        &self.components
    }

    /// Mutable access to low-level registered base components.
    pub fn components_mut(&mut self) -> &mut Vec<LocatedComponent> {
        &mut self.components
    }

    /// Constructs a new `Frame` populated with all base components registered in this model.
    pub fn create_frame(&self) -> Frame {
        let mut frame = Frame::new(self.size);
        for comp in &self.components {
            let mut comp = comp.clone();
            comp.sync_location();
            frame.content.add(comp);
        }
        frame
    }

    /// Low-level access to push a pre-constructed frame directly into the underlying scene.
    pub fn push_frame(&mut self, frame: Frame) -> Result<(), RenderError> {
        self.scene.push_frame(frame)
    }

    /// Read-only access to the low-level `Scene`.
    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    /// Mutable access to the low-level `Scene`.
    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    /// Consumes the `Model` and returns the inner `Scene`.
    pub fn into_scene(self) -> Scene {
        self.scene
    }

    /// Mutate a component behind an `ElementPointer` directly.
    pub fn with_pointer<C: 'static, R, F>(
        &self,
        pointer: ElementPointer<C>,
        f: F,
    ) -> Result<R, RenderError>
    where
        F: FnOnce(&mut C) -> R,
    {
        pointer.with_mut(f)
    }

    // =========================================================================
    // High-Level Generic Animation & Timing Control
    // =========================================================================

    /// Animates a generic target object by passing a mutable reference `&mut T` on every step (Linear easing).
    ///
    /// CRITICAL: For each frame step, the target modification occurs FIRST, and the frame
    /// is captured/added to the scene IMMEDIATELY before the next modification occurs.
    pub fn animate<T, F>(
        &mut self,
        target: &mut T,
        duration_sec: f64,
        anim_fn: F,
    ) -> Result<(), RenderError>
    where
        F: FnMut(&mut T, f64),
    {
        self.animate_easing(target, duration_sec, Easing::Linear, anim_fn)
    }

    /// Animates a generic target object with a specified `Easing` curve.
    ///
    /// CRITICAL: Ensures that for each frame step, target modification occurs FIRST,
    /// and the frame is captured/added to the scene IMMEDIATELY before the next modification occurs,
    /// since components hold pointers rather than owned snapshot objects.
    pub fn animate_easing<T, F>(
        &mut self,
        target: &mut T,
        duration_sec: f64,
        easing: Easing,
        mut anim_fn: F,
    ) -> Result<(), RenderError>
    where
        F: FnMut(&mut T, f64),
    {
        let fps_val = self.scene.fps().value() as f64;
        let total_frames = (duration_sec * fps_val).round() as u32;
        let total_frames = total_frames.max(1);

        for i in 0..total_frames {
            let linear_t = if total_frames == 1 {
                1.0
            } else {
                i as f64 / (total_frames - 1) as f64
            };
            let eased_t = easing.ease(linear_t);

            // 1. Modify the target object FIRST
            anim_fn(target, eased_t);

            // 2. Create and push frame IMMEDIATELY before the next modification occurs
            let frame = self.create_frame();
            self.scene.push_frame(frame)?;
        }

        Ok(())
    }

    /// Animates a component behind an `ElementPointer<C>`.
    ///
    /// CRITICAL: Ensures that for each frame step, the component behind the pointer is modified FIRST,
    /// and the frame is captured/added to the scene IMMEDIATELY before the next modification occurs.
    pub fn animate_pointer<C, F>(
        &mut self,
        pointer: ElementPointer<C>,
        duration_sec: f64,
        easing: Easing,
        mut anim_fn: F,
    ) -> Result<(), RenderError>
    where
        C: Component + 'static,
        F: FnMut(&mut C, f64),
    {
        self.animate_easing(&mut (), duration_sec, easing, |_dummy, progress| {
            let _ = pointer.with_mut(|comp| {
                anim_fn(comp, progress);
            });
        })
    }

    /// Holds current state static for a specified duration in seconds.
    pub fn wait(&mut self, duration_sec: f64) -> Result<(), RenderError> {
        let fps_val = self.scene.fps().value() as f64;
        let total_frames = (duration_sec * fps_val).round() as u32;
        for _ in 0..total_frames {
            let frame = self.create_frame();
            self.scene.push_frame(frame)?;
        }
        Ok(())
    }

    /// Renders the entire animation scene to a video file at `path`.
    pub fn render_video(&self, path: &Path) -> Result<(), RenderError> {
        self.scene.render(path)
    }
}

// =============================================================================
// Frame-by-Frame Scene & Model Multiplication (Composition)
// =============================================================================

impl Mul<Model> for Model {
    type Output = Scene;
    fn mul(self, rhs: Model) -> Self::Output {
        multiply_scenes(self.scene(), rhs.scene())
    }
}

impl Mul<&Model> for Model {
    type Output = Scene;
    fn mul(self, rhs: &Model) -> Self::Output {
        multiply_scenes(self.scene(), rhs.scene())
    }
}

impl Mul<Model> for &Model {
    type Output = Scene;
    fn mul(self, rhs: Model) -> Self::Output {
        multiply_scenes(self.scene(), rhs.scene())
    }
}

impl Mul<&Model> for &Model {
    type Output = Scene;
    fn mul(self, rhs: &Model) -> Self::Output {
        multiply_scenes(self.scene(), rhs.scene())
    }
}

impl Mul<Scene> for Model {
    type Output = Scene;
    fn mul(self, rhs: Scene) -> Self::Output {
        multiply_scenes(self.scene(), &rhs)
    }
}

impl Mul<&Scene> for Model {
    type Output = Scene;
    fn mul(self, rhs: &Scene) -> Self::Output {
        multiply_scenes(self.scene(), rhs)
    }
}

impl Mul<Scene> for &Model {
    type Output = Scene;
    fn mul(self, rhs: Scene) -> Self::Output {
        multiply_scenes(self.scene(), &rhs)
    }
}

impl Mul<&Scene> for &Model {
    type Output = Scene;
    fn mul(self, rhs: &Scene) -> Self::Output {
        multiply_scenes(self.scene(), rhs)
    }
}

impl Mul<Model> for Scene {
    type Output = Scene;
    fn mul(self, rhs: Model) -> Self::Output {
        multiply_scenes(&self, rhs.scene())
    }
}

impl Mul<&Model> for Scene {
    type Output = Scene;
    fn mul(self, rhs: &Model) -> Self::Output {
        multiply_scenes(&self, rhs.scene())
    }
}

impl Mul<Model> for &Scene {
    type Output = Scene;
    fn mul(self, rhs: Model) -> Self::Output {
        multiply_scenes(self, rhs.scene())
    }
}

impl Mul<&Model> for &Scene {
    type Output = Scene;
    fn mul(self, rhs: &Model) -> Self::Output {
        multiply_scenes(self, rhs.scene())
    }
}

// =============================================================================
// Lua Binding / UserData Compatibility
// =============================================================================

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

fn parse_easing_val(val: mlua::Value) -> Easing {
    if let mlua::Value::UserData(ud) = &val {
        if let Ok(e) = ud.borrow::<Easing>() {
            return *e;
        }
    }
    if let mlua::Value::String(s) = &val {
        if let Ok(str_val) = s.to_str() {
            match str_val.as_ref() {
                "ease_in" => return Easing::EaseIn,
                "ease_out" => return Easing::EaseOut,
                "ease_in_out" => return Easing::EaseInOut,
                _ => return Easing::Linear,
            }
        }
    }
    Easing::Linear
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
            down_cast!(this, add_component, val);
            Ok(())
        });

        methods.add_method_mut(
            "spawn_located",
            |_, this, (val, loc): (mlua::AnyUserData, Location)| {
                let located  = down_cast!(val, |c| {
                    LocatedComponent {
                        child: Box::new(c),
                        location: loc.clone(),
                    }
                }).ok_or_else(|| {mlua::Error::external(format!("Failed to downcast commponent"))});

                this.spawn(located?).map_err(|e| mlua::Error::external(format!("{:?}", e)))
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
                })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{
        components::{
            freebox::{Coordinate, LocatedComponent, Location},
            text::Text,
        },
        models::Size,
    };

    #[test]
    fn test_model_spawn_and_animate_clean_api() -> Result<(), RenderError> {
        let mut model = Model::new(Size::youtube_shorts(), FpsTarget::F120);

        let mut model2 = Model::new(Size::youtube_shorts(), FpsTarget::F120);
        model2.wait(5.0)?;
        // Single line spawning without managing multiple placeholder/pointer variables!
        let controller = model.spawn_located(
            Text::new("Clean Model API").size(40).color("#ffffff"),
            Location::new(
                Coordinate::with_progress(Coordinate::zero(), Coordinate::Percentage(0.5)),
                Coordinate::with_progress(Coordinate::zero(), Coordinate::Percentage(0.5)),
            ),
        )?;
        model.with_pointer(controller, |a| {
            a.set_anchor(crate::render::components::freebox::Anchor::Center);
        })?;
        // Animate pointer using generic animate_pointer with Easing
        model.animate_pointer(controller, 1.0, Easing::EaseInOut, |lc, progress| {
            lc.location.set_progress(progress).unwrap();
        })?;

        let s = model2 * model;

        let output_dir = Path::new("./tests");
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).ok();
        }
        let output_path = output_dir.join("test_model_animation.mp4");
        s.render(&output_path)?;

        assert!(output_path.exists());
        Ok(())
    }

    #[test]
    fn test_model_generic_animate_mutable_object() -> Result<(), RenderError> {
        let mut model = Model::new(Size::youtube_shorts(), FpsTarget::F60);

        struct State {
            text_ptr: ElementPointer<LocatedComponent>,
            x_pos: f64,
        }

        let ptr = model.spawn_located(
            Text::new("Generic Target Test").size(32).color("#00ff00"),
            Location::new(Coordinate::absolute(0.0), Coordinate::absolute(100.0)),
        )?;

        let mut state = State {
            text_ptr: ptr,
            x_pos: 0.0,
        };

        // Animate generic mutable target state
        model.animate(&mut state, 0.5, |st, progress| {
            st.x_pos = progress * 300.0;
            let current_x = st.x_pos;
            let _ = st.text_ptr.with_mut(|lc| {
                lc.set_location(Location::new(
                    Coordinate::Absolute(current_x),
                    Coordinate::Absolute(100.0),
                ));
            });
        })?;

        assert!(model.scene().frames_count() >= 30);
        Ok(())
    }

    #[test]
    fn test_model_and_scene_multiplication() -> Result<(), RenderError> {
        let mut model_a = Model::new(Size::youtube_shorts(), FpsTarget::F30);
        model_a.add_component(Text::new("Base Layer").size(30).color("#ff0000"));
        model_a.wait(0.1)?; // 3 frames

        let mut model_b = Model::new(Size::youtube_shorts(), FpsTarget::F30);
        model_b.add_component(Text::new("Overlay Layer").size(30).color("#0000ff"));
        model_b.wait(0.1)?; // 3 frames

        // Equal length multiplication
        let scene_combined = &model_a * &model_b;
        assert_eq!(scene_combined.frames_count(), 3);

        // Unequal length multiplication (e.g. 3 frames * 1 frame)
        let mut model_single = Model::new(Size::youtube_shorts(), FpsTarget::F30);
        model_single.add_component(Text::new("Single Frame").size(30).color("#00ff00"));
        model_single.wait(0.033)?; // 1 frame

        let scene_unequal = &model_a * &model_single;
        assert_eq!(scene_unequal.frames_count(), 3);

        let scene_unequal_reverse = &model_single * &model_a;
        assert_eq!(scene_unequal_reverse.frames_count(), 3);

        Ok(())
    }
}
