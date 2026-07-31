use std::path::Path;

use crate::render::{
    animation::FpsTarget,
    components::{
        freebox::{Coordinate, LocatedComponent, Location},
        placeholder::{ElementPointer, PlaceHolder},
    },
    error::RenderError,
    frame::{Component, Frame},
    models::Size,
    scene::Scene,
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

/// `Model` offers high-level animation, composition, timing, and frame synchronization control,
/// while maintaining low-level access to components and underlying `Scene` structures.
///
/// Designed with full compatibility for Lua bindings (via `mlua::UserData`).
pub struct Model {
    size: Size,
    scene: Scene,
    components: Vec<LocatedComponent>,
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
            frame.content.add(comp.clone());
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
// Lua Binding / UserData Compatibility
// =============================================================================

impl mlua_extras::mlua::UserData for Model {
    fn add_methods<M: mlua_extras::mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("size", |_lua, this, ()| {
            Ok((this.size().width, this.size().height))
        });

        methods.add_method_mut("set_size", |_lua, this, (w, h): (f64, f64)| {
            this.set_size(Size::new(w, h));
            Ok(())
        });

        methods.add_method("fps", |_lua, this, ()| Ok(this.fps().value()));

        methods.add_method_mut("set_fps", |_lua, this, fps: u32| {
            this.set_fps(FpsTarget::Custom(fps));
            Ok(())
        });

        methods.add_method_mut("wait", |_lua, this, duration_sec: f64| {
            this.wait(duration_sec)
                .map_err(|e| mlua_extras::mlua::Error::external(format!("{:?}", e)))
        });

        methods.add_method("render_video", |_lua, this, path_str: String| {
            this.render_video(Path::new(&path_str))
                .map_err(|e| mlua_extras::mlua::Error::external(format!("{:?}", e)))
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

        // Single line spawning without managing multiple placeholder/pointer variables!
        let controller = model.spawn_located(
            Text::new("Clean Model API").size(40).color("#ffffff"),
            Location::new(
                Coordinate::with_progress(Coordinate::zero(), Coordinate::Percentage(0.5)),
                Coordinate::with_progress(Coordinate::zero(), Coordinate::Percentage(0.5)),
            ),
        )?;

        // Animate pointer using generic animate_pointer with Easing
        model.animate_pointer(controller, 1.0, Easing::EaseInOut, |lc, progress| {
            lc.location.set_progress(progress).unwrap();
        })?;

        let output_dir = Path::new("./tests");
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).ok();
        }
        let output_path = output_dir.join("test_model_animation.mp4");
        model.render_video(&output_path)?;

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
                lc.shift(Location::new(
                    Coordinate::Absolute(current_x),
                    Coordinate::Absolute(100.0),
                ));
            });
        })?;

        assert!(model.scene().frames_count() >= 30);
        Ok(())
    }
}
