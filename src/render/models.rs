#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn x_landscape() -> Self {
        Self::new(1200, 675)
    }

    pub fn tiktok() -> Self {
        Self::new(1080, 1920)
    }

    pub fn instagram_reels() -> Self {
        Self::new(1080, 1920)
    }

    pub fn instagram_portrait() -> Self {
        Self::new(1080, 1350)
    }

    pub fn instagram_square() -> Self {
        Self::new(1080, 1080)
    }

    pub fn youtube_shorts() -> Self {
        Self::new(1080, 1920)
    }

    pub fn youtube_video() -> Self {
        Self::new(1920, 1080)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub position: Position,
    pub size: Size,
}

impl Bounds {
    pub fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_tiny_skia(self) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(self.r, self.g, self.b, self.a).unwrap_or(tiny_skia::Color::BLACK)
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerModel {
    pub id: String,
    pub z_index: i32,
    pub visible: bool,
}

impl LayerModel {
    pub fn new(id: impl Into<String>, z_index: i32) -> Self {
        Self {
            id: id.into(),
            z_index,
            visible: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneModel {
    pub size: Size,
    pub layers: Vec<LayerModel>,
}

impl SceneModel {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(mut self, layer: LayerModel) -> Self {
        self.layers.push(layer);
        self
    }
}
