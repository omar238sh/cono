use taffy::NodeId;
use typst::{text::FontFamily, visualize::Color};


pub struct Text {
    id: NodeId,
    content: String,
    color: Color,
    font: String,
    size: f64,
}
