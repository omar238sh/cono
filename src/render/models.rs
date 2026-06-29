#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn x_landscape() -> Self {
        Self::new(1200.0, 675.0)
    }

    pub fn tiktok() -> Self {
        Self::new(1080.0, 1920.0)
    }

    pub fn instagram_reels() -> Self {
        Self::new(1080.0, 1920.0)
    }

    pub fn instagram_portrait() -> Self {
        Self::new(1080.0, 1350.0)
    }

    pub fn instagram_square() -> Self {
        Self::new(1080.0, 1080.0)
    }

    pub fn youtube_shorts() -> Self {
        Self::new(1080.0, 1920.0)
    }

    pub fn youtube_video() -> Self {
        Self::new(1920.0, 1080.0)
    }
}
