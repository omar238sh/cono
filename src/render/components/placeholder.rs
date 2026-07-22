use std::{
    marker::PhantomData,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use taffy::NodeId;

use crate::render::{
    components::placeholder::RegisterMod::Register, error::RenderError, frame::Component,
    models::Size,
};

#[derive(Debug)]
pub struct PlaceHolder<C: Clone + 'static> {
    pointer: ElementPointer<C>,
}

impl<C: Clone + Component + 'static> Clone for PlaceHolder<C> {
    fn clone(&self) -> Self {
        let element = self.pointer.with(|e| e.clone()).unwrap();
        let a = PlaceHolder::from(element).unwrap();
        a
    }
}

impl<C: Component + Clone + 'static> PlaceHolder<C> {
    pub fn from(c: C) -> Result<Self, RenderError> {
        let controller = GLOBAL_REGISTER
            .lock()
            .map_err(|_| {
                RenderError::CustomError(
                    "Lock Error : You Cannot Do any thing about this".to_string(),
                )
            })?
            .add(c)?;
        Ok(Self {
            pointer: controller,
        })
    }

    pub fn pointer(&self) -> ElementPointer<C> {
        self.pointer
    }
}

static GLOBAL_REGISTER: LazyLock<Mutex<Register>> = LazyLock::new(|| Mutex::new(Register::new()));

impl<C: Component + Clone + 'static> Component for PlaceHolder<C> {
    fn render(
        &self,
        tree: &taffy::TaffyTree,
    ) -> Result<vello::Scene, crate::render::error::RenderError> {
        self.pointer.with(|s| s.render(tree))?
    }

    fn size(&self) -> std::result::Result<Size, RenderError> {
        self.pointer.with(|s| s.size())?
    }

    fn register_layout(
        &mut self,
        tree: &mut taffy::TaffyTree,
    ) -> Result<NodeId, crate::render::error::RenderError> {
        self.pointer.with_mut(|s| s.register_layout(tree))?
    }

    fn layout(&self) -> Result<NodeId, crate::render::error::RenderError> {
        self.pointer.with(|s| s.layout())?
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ElementPointer<T> {
    id: ElementId,
    _type: PhantomData<T>,
}

impl<T: 'static> Clone for ElementPointer<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for ElementPointer<T> {}

impl<T: 'static> ElementPointer<T> {
    pub fn new(id: ElementId) -> Self {
        Self {
            id: id,
            _type: PhantomData::default(),
        }
    }

    pub fn with<R, F>(&self, f: F) -> Result<R, RenderError>
    where
        F: FnOnce(&T) -> R,
    {
        let register_guard = GLOBAL_REGISTER.lock().map_err(|_| {
            RenderError::CustomError("Lock Error : You Cannot Do any thing about this".to_string())
        })?;

        let real_component = register_guard
            .get(&self.id)?
            .as_any()
            .downcast_ref::<T>()
            .unwrap();
        Ok(f(real_component))
    }

    pub fn with_mut<R, F>(&self, f: F) -> Result<R, RenderError>
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut register_guard = GLOBAL_REGISTER.lock().map_err(|_| {
            RenderError::CustomError("Lock Error : You Cannot Do any thing about this".to_string())
        })?;

        let real_component = register_guard
            .get_mut(&self.id)?
            .as_any_mut()
            .downcast_mut::<T>()
            .unwrap();
        Ok(f(real_component))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(u64);

impl ElementId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

pub mod RegisterMod {
    use std::collections::HashMap;

    use crate::render::{
        components::placeholder::{ElementId, ElementPointer},
        error::RenderError,
        frame::Component,
    };

    pub struct Register {
        contents: HashMap<ElementId, Box<dyn Component>>,
    }

    impl Register {
        pub fn new() -> Self {
            Self {
                contents: HashMap::new(),
            }
        }

        pub fn add<C: Component + 'static>(
            &mut self,
            c: C,
        ) -> Result<ElementPointer<C>, RenderError> {
            let id = ElementId::new();
            self.contents.insert(id, Box::new(c));
            Ok(ElementPointer::new(id))
        }

        pub fn get(&self, id: &ElementId) -> Result<&Box<dyn Component>, RenderError> {
            self.contents.get(&id).ok_or(RenderError::CustomError(
                "Cannot Find Component in Register".to_string(),
            ))
        }

        pub fn get_mut(&mut self, id: &ElementId) -> Result<&mut Box<dyn Component>, RenderError> {
            self.contents.get_mut(&id).ok_or(RenderError::CustomError(
                "Cannot Find Component in Register".to_string(),
            ))
        }
    }
}
