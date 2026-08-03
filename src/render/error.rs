use taffy::TaffyError;
#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("failed to parse SVG: {0}")]
    SVGParseError(#[from] vello_svg::usvg::Error),
    #[error("failed to render SVG with vello: {0}")]
    VelloSvgError(#[from] vello_svg::Error),
    #[error("{0}")]
    CustomError(String),
    #[error("{0}")]
    LayoutError(#[from] TaffyError),
    #[error("Component doesn't have id in the tree")]
    NotComputed,
    #[error("typst errors {0}")]
    TypstError(String),
    #[error("Compiler access error")]
    TypstCompilerError,
    #[error("Reach the last frame")]
    FrameOverFlow,
    #[error("FPS dismatch")]
    FpsDismatch,
    #[error("Cannot init ffmpef")]
    FfmpegError,
}
