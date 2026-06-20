#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("failed to parse SVG: {0}")]
    SVGParseError(#[from] vello_svg::usvg::Error),
    #[error("failed to render SVG with vello: {0}")]
    VelloSvgError(#[from] vello_svg::Error),
    #[error("{0}")]
    CustomError(String),

}
