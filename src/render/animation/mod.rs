pub mod animengin;
pub mod model;

pub use model::{Easing, Model};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FpsTarget {
    F30,
    F45,
    F60,
    F120,
    Custom(u32),
}

impl FpsTarget {
    pub fn value(&self) -> u32 {
        match self {
            FpsTarget::F30 => 30,
            FpsTarget::F45 => 45,
            FpsTarget::F60 => 60,
            FpsTarget::F120 => 120,
            FpsTarget::Custom(v) => v.clone(),
        }
    }
}
