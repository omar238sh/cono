#[derive(Debug, Clone, Copy)]
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
