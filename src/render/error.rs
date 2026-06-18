#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("failed to render SVG: {0}")]
    SVGError(#[from] usvg::Error),
    #[error("{0}")]
    CustomError(String),
    #[error("failed to create pixmap")]
    CannotCreatePixmap,
}
