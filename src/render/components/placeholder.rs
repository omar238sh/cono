use std::{
    marker::PhantomData,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use taffy::NodeId;

use crate::render::{
    components::{freebox::Location, placeholder::RegisterMod::Register},
    error::RenderError,
    frame::Component,
    models::Size,
};

#[derive(Debug)]
pub struct PlaceHolder<C: 'static> {
    pub pointer: ElementPointer<C>,
}

impl<C: Component + Clone + 'static> mlua::FromLua for PlaceHolder<C> {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "PlaceHolder".to_string(),
            message: Some("expected PlaceHolder UserData".to_string()),
        })
    }
}

// impl<C: Component + 'static> Clone for PlaceHolder<C> {
//     fn clone(&self) -> Self {
//         Self {
//             pointer: self.pointer,
//         }
//     }
// }

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

impl<C: Component + 'static + Clone> Clone for PlaceHolder<C> {
    fn clone(&self) -> Self {
        Self::from(self.pointer.with(|a| a.clone()).unwrap()).unwrap()
    }
}

pub(crate) static GLOBAL_REGISTER: LazyLock<Mutex<Register>> =
    LazyLock::new(|| Mutex::new(Register::new()));

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

    fn get_location(&self) -> Option<Location> {
        self.pointer.with(|s| s.get_location()).unwrap_or(None)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    // fn locate(self) -> LocatedComponent
    // where
    //     Self: Sized + 'static,
    // {
    //     let register_guard = GLOBAL_REGISTER
    //         .lock()
    //         .map_err(|_| {
    //             RenderError::CustomError(
    //                 "Lock Error : You Cannot Do any thing about this".to_string(),
    //             )
    //         })
    //         .unwrap();

    //     let real_component = register_guard
    //         .get(&self.pointer.id)
    //         .unwrap()
    //         .as_any()
    //         .downcast_ref::<LocatedComponent>();

    //     match real_component {
    //         Some(v) => LocatedComponent {
    //             child: Box::new(self),
    //             location: v.get_location(),
    //         },
    //         None => LocatedComponent::new(self),
    //     }
    // }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ElementPointer<T> {
    pub id: ElementId,
    _type: PhantomData<T>,
}

impl<T: 'static> mlua::FromLua for ElementPointer<T> {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(*ud.borrow::<Self>()?);
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "ElementPointer".to_string(),
            message: Some("expected ElementPointer UserData".to_string()),
        })
    }
}

impl<T> Clone for ElementPointer<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElementPointer<T> {}

impl<T: 'static> ElementPointer<T> {
    pub fn new(id: ElementId) -> Self {
        Self {
            id: id,
            _type: PhantomData::default(),
        }
    }

    pub fn drop_component(&self) {
        let mut register_guard = GLOBAL_REGISTER
            .lock()
            .map_err(|_| {
                RenderError::CustomError(
                    "Lock Error : You Cannot Do any thing about this".to_string(),
                )
            })
            .unwrap(); // if it panic it's a bug

        register_guard.remove(&self.id);
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

use crate::render::components::{
    column::Column,
    container::Container,
    freebox::{FreeBox, LocatedComponent},
    row::Row,
    svg::SvgBox,
    text::Text,
};

pub fn invoke_lua_with_component_mut(
    lua: &mlua::Lua,
    comp: &mut dyn Component,
    func: mlua::Function,
) -> mlua::Result<()> {
    if let Some(lc) = comp.as_any_mut().downcast_mut::<LocatedComponent>() {
        let mut temp = lc.clone();
        func.call::<()>(temp.clone())?;
        *lc = temp;
        return Ok(());
    }
    if let Some(t) = comp.as_any_mut().downcast_mut::<Text>() {
        let mut temp = t.clone();
        func.call::<()>(temp.clone())?;
        *t = temp;
        return Ok(());
    }
    if let Some(c) = comp.as_any_mut().downcast_mut::<Column>() {
        let mut temp = c.clone();
        func.call::<()>(temp.clone())?;
        *c = temp;
        return Ok(());
    }
    if let Some(r) = comp.as_any_mut().downcast_mut::<Row>() {
        let mut temp = r.clone();
        func.call::<()>(temp.clone())?;
        *r = temp;
        return Ok(());
    }
    if let Some(cnt) = comp.as_any_mut().downcast_mut::<Container>() {
        let mut temp = cnt.clone();
        func.call::<()>(temp.clone())?;
        *cnt = temp;
        return Ok(());
    }
    if let Some(f) = comp.as_any_mut().downcast_mut::<FreeBox>() {
        let mut temp = f.clone();
        func.call::<()>(temp.clone())?;
        *f = temp;
        return Ok(());
    }
    if let Some(s) = comp.as_any_mut().downcast_mut::<SvgBox>() {
        let mut temp = s.clone();
        func.call::<()>(temp.clone())?;
        *s = temp;
        return Ok(());
    }
    Err(mlua::Error::external(
        "Unknown component type in ElementPointer::with_mut",
    ))
}

pub fn invoke_lua_with_component(
    lua: &mlua::Lua,
    comp: &dyn Component,
    func: mlua::Function,
) -> mlua::Result<()> {
    if let Some(lc) = comp.as_any().downcast_ref::<LocatedComponent>() {
        return func.call::<()>(lc.clone());
    }
    if let Some(t) = comp.as_any().downcast_ref::<Text>() {
        return func.call::<()>(t.clone());
    }
    if let Some(c) = comp.as_any().downcast_ref::<Column>() {
        return func.call::<()>(c.clone());
    }
    if let Some(r) = comp.as_any().downcast_ref::<Row>() {
        return func.call::<()>(r.clone());
    }
    if let Some(cnt) = comp.as_any().downcast_ref::<Container>() {
        return func.call::<()>(cnt.clone());
    }
    if let Some(f) = comp.as_any().downcast_ref::<FreeBox>() {
        return func.call::<()>(f.clone());
    }
    if let Some(s) = comp.as_any().downcast_ref::<SvgBox>() {
        return func.call::<()>(s.clone());
    }
    Err(mlua::Error::external(
        "Unknown component type in ElementPointer::with",
    ))
}

impl<T: 'static> mlua::UserData for ElementPointer<T> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id.0));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("drop_component", |_, this, ()| {
            this.drop_component();
            Ok(())
        });

        methods.add_method("with", |lua, this, func: mlua::Function| {
            let register_guard = GLOBAL_REGISTER
                .lock()
                .map_err(|_| mlua::Error::external("Lock Error: GLOBAL_REGISTER"))?;
            let box_comp = register_guard
                .get(&this.id)
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))?;
            invoke_lua_with_component(lua, &**box_comp, func)
        });

        methods.add_method_mut("with_mut", |lua, this, func: mlua::Function| {
            let mut register_guard = GLOBAL_REGISTER
                .lock()
                .map_err(|_| mlua::Error::external("Lock Error: GLOBAL_REGISTER"))?;
            let box_comp = register_guard
                .get_mut(&this.id)
                .map_err(|e| mlua::Error::external(format!("{:?}", e)))?;
            invoke_lua_with_component_mut(lua, &mut **box_comp, func)
        });
    }
}

impl<C: Component + Clone + 'static> mlua::UserData for PlaceHolder<C> {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("pointer", |_, this, ()| Ok(this.pointer));
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

        pub fn remove(&mut self, id: &ElementId) {
            self.contents.remove(&id).unwrap();
        }
    }
}
