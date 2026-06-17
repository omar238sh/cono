#[derive(thiserror::Error, Debug)]
pub enum RenderError<'a> {
    #[error("failed to render SVG: {0}")]
    SVGError(#[from] usvg::Error),
    #[error("{0}")]
    CustomError(&'a str),
    #[error("failed to create pixmap")]
    CannotCreatePixmap,
}
