use taffy::LengthPercentage;

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
        self.size
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

#[derive(Clone)]
pub enum Alignment {
    Start,
    End,
    Center,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
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

    pub fn to_rect(&self) -> taffy::Rect<LengthPercentage> {
        taffy::Rect {
            left: LengthPercentage::length(self.left as f32),
            right: LengthPercentage::length(self.right as f32),
            top: LengthPercentage::length(self.top as f32),
            bottom: LengthPercentage::length(self.bottom as f32),
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
