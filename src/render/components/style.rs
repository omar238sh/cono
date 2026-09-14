use taffy::LengthPercentageAuto;

use crate::render::models::Size;

#[derive(Clone)]
pub struct Style {
    size: Size,
    arrangement: Arrangement,
    alignment: Alignment,
    padding: Padding,
}

impl Style {
    pub fn new(size: Size) -> Self {
        Self {
            size: size,
            arrangement: Arrangement::Start,
            alignment: Alignment::Start,
            padding: Padding::new(0f64),
        }
    }

    pub fn arrangement_mut(&mut self) -> &mut Arrangement {
        &mut self.arrangement
    }

    pub fn arrangement(&self) -> &Arrangement {
        &self.arrangement
    }

    pub fn size(&self) -> Size {
        let s = self.size;
        Size {
            width: s.width + self.padding.left + self.padding.right,
            height: s.height + self.padding.top + self.padding.bottom,
        }
    }

    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }

    pub fn alignment_mut(&mut self) -> &mut Alignment {
        &mut self.alignment
    }

    pub fn padding(&self) -> &Padding {
        &self.padding
    }

    pub fn padding_mut(&mut self) -> &mut Padding {
        &mut self.padding
    }
}
#[derive(Clone)]
pub enum Arrangement {
    SpaceBetween,
    SpacedBy(u32),
    Start,
    End,
    SpaceAround,
    SpaceEvenly,
    Center,
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Start,
    End,
    Center,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub(crate) left: f64,
    pub(crate) right: f64,
    pub(crate) top: f64,
    pub(crate) bottom: f64,
}

impl Padding {
    pub fn new(value: f64) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn edit(&mut self, value: f64) {
        self.left = value;
        self.right = value;
        self.top = value;
        self.bottom = value;
    }

    pub fn to_rect(&self) -> taffy::Rect<LengthPercentageAuto> {
        taffy::Rect {
            left: LengthPercentageAuto::length(self.left as f32),
            right: LengthPercentageAuto::length(self.right as f32),
            top: LengthPercentageAuto::length(self.top as f32),
            bottom: LengthPercentageAuto::length(self.bottom as f32),
        }
    }

    pub fn left(mut self, value: f64) -> Self {
        self.left = value;
        self
    }

    pub fn right(mut self, value: f64) -> Self {
        self.right = value;
        self
    }

    pub fn top(mut self, value: f64) -> Self {
        self.top = value;
        self
    }

    pub fn bottom(mut self, value: f64) -> Self {
        self.bottom = value;
        self
    }
}
