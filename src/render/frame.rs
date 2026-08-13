use std::any::Any;

use image::{ImageBuffer, Rgba};
use taffy::{NodeId, TaffyTree};
use vello::Scene;

use crate::down_cast;
use crate::render::components::SceneGpuRenderer;
use crate::render::components::freebox::{Angle, FreeBox, LocatedComponent, Location, Opacity};
use crate::render::components::placeholder::PlaceHolder;
use crate::render::error::RenderError;
use crate::render::models::Size;

#[derive(Clone)]
pub struct Frame {
    pub content: FreeBox,
}

impl mlua::FromLua for Frame {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if let mlua::Value::UserData(ud) = value {
            return Ok(ud.borrow::<Self>()?.clone());
        }
        Err(mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "Frame".to_string(),
            message: Some("expected Frame UserData".to_string()),
        })
    }
}

impl Frame {
    pub fn new(size: Size) -> Self {
        Self {
            content: FreeBox::new(size),
        }
    }

    pub fn add<C: Component + 'static>(mut self, c: C) -> Self {
        self.content = self.content.with_child(c.locate());
        self
    }

    pub fn add_mut<C: Component + 'static>(&mut self, c: C) {
        self.content.add(c.locate());
    }

    pub fn to_image(&mut self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, RenderError> {
        let mut tree = TaffyTree::new();
        self.content.register_layout(&mut tree)?;
        let mut renderer = SceneGpuRenderer::new()?;
        let image = renderer.render_to_image(
            &self.content.render(&tree)?,
            self.content.size()?.width as u32,
            self.content.size()?.height as u32,
        )?;
        Ok(image)
    }
}
impl mlua::UserData for Frame {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("content", |_, this| Ok(this.content.clone()));
        fields.add_field_method_set("content", |_, this, content: FreeBox| {
            this.content = content;
            Ok(())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_, size: Size| Ok(Frame::new(size)));
        methods.add_method_mut("add", |_, this, val: mlua::AnyUserData| {
            down_cast!(this, add_mut, val => [
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
    }
}

// static Tree: LazyLock<Mutex<TaffyTree>> = LazyLock::new(|| Mutex::new(TaffyTree::new()));

pub trait Component: ComponentClone + Send + 'static {
    fn render(&self, tree: &TaffyTree) -> Result<Scene, RenderError>;

    fn size(&self) -> Result<Size, RenderError>;

    fn computed_size(&self, tree: &TaffyTree) -> Result<Size, RenderError> {
        let id = self.layout()?;
        let layout = tree.layout(id)?;

        let size = Size::new(layout.size.width as f64, layout.size.height as f64);

        Ok(size)
    }

    fn register_layout(&mut self, tree: &mut TaffyTree) -> Result<NodeId, RenderError>;
    fn layout(&self) -> Result<NodeId, RenderError>;

    fn get_location(&self) -> Option<Location> {
        None
    }

    fn get_angle(&self) -> Option<Angle> {
        None
    }

    fn get_opacity(&self) -> Option<Opacity> {
        None
    }

    fn locate(self) -> LocatedComponent
    where
        Self: Sized + 'static,
    {
        LocatedComponent::new(self)
    }

    fn to_placeholder(self) -> Result<PlaceHolder<Self>, RenderError>
    where
        Self: Sized + Clone,
    {
        PlaceHolder::from(self)
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
    T: Component + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// impl Component for Box<dyn Component> {
//     fn render(&self, tree: &TaffyTree) -> Result<Scene, RenderError> {
//         self.as_ref().render(tree)
//     }

//     fn size(&self) -> Result<Size, RenderError> {
//         self.as_ref().size()
//     }

//     fn register_layout(&mut self, tree: &mut TaffyTree) -> Result<NodeId, RenderError> {
//         self.as_mut().register_layout(tree)
//     }

//     fn layout(&self) -> Result<NodeId, RenderError> {
//         self.as_ref().layout()
//     }

//     fn get_location(&self) -> Option<Location> {
//         self.as_ref().get_location()
//     }

//     fn locate(self) -> LocatedComponent {
//         LocatedComponent {
//             child: self,
//             location: Location::zero(),
//         }
//     }

//     fn as_any(&self) -> &dyn Any {
//         self.as_ref().as_any()
//     }

//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self.as_mut().as_any_mut()
//     }
// }
