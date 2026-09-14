use taffy::{
    AlignItems, Display, JustifyContent, NodeId, TaffyTree,
    prelude::{fr, length},
};
use vello::{Scene, kurbo::Affine};

use crate::render::{
    components::{
        freebox::{FreeBox, LocatedComponent, Location},
        style::{Alignment, Arrangement, Padding, Style},
    },
    error::RenderError,
    frame::Component,
    models::Size,
};

#[derive(Clone)]
pub struct Grid {
    pub(crate) contents: Vec<Box<dyn Component>>,
    id: Option<NodeId>,
    pub(crate) style: Style,
    pub(crate) cols: usize,
    pub(crate) rows: Option<usize>,
    pub(crate) col_gap: f64,
    pub(crate) row_gap: f64,
}

impl Grid {
    pub fn new(size: Size, cols: usize) -> Self {
        Self {
            contents: Vec::new(),
            id: None,
            style: Style::new(size),
            cols: cols.max(1),
            rows: None,
            col_gap: 0.0,
            row_gap: 0.0,
        }
    }

    pub fn from_size(width: f64, height: f64, cols: usize) -> Self {
        Self::new(Size::new(width, height), cols)
    }

    pub fn push<T: Component + 'static>(&mut self, component: T) {
        self.contents.push(Box::new(component));
    }

    pub fn cols(mut self, cols: usize) -> Self {
        self.cols = cols.max(1);
        self
    }

    pub fn set_cols(&mut self, cols: usize) {
        self.cols = cols.max(1);
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows.max(1));
        self
    }

    pub fn set_rows(&mut self, rows: usize) {
        self.rows = Some(rows.max(1));
    }

    pub fn gap(mut self, gap: f64) -> Self {
        self.col_gap = gap;
        self.row_gap = gap;
        self
    }

    pub fn set_gap(&mut self, gap: f64) {
        self.col_gap = gap;
        self.row_gap = gap;
    }

    pub fn col_gap(mut self, gap: f64) -> Self {
        self.col_gap = gap;
        self
    }

    pub fn set_col_gap(&mut self, gap: f64) {
        self.col_gap = gap;
    }

    pub fn row_gap(mut self, gap: f64) -> Self {
        self.row_gap = gap;
        self
    }

    pub fn set_row_gap(&mut self, gap: f64) {
        self.row_gap = gap;
    }

    pub fn arrangment(mut self, arrangement: Arrangement) -> Self {
        *self.style.arrangement_mut() = arrangement;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        *self.style.alignment_mut() = alignment;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        *self.style.padding_mut() = padding;
        self
    }

    pub fn set_arrangment(&mut self, arrangement: Arrangement) {
        *self.style.arrangement_mut() = arrangement;
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        *self.style.alignment_mut() = alignment;
    }

    pub fn set_padding(&mut self, padding: Padding) {
        *self.style.padding_mut() = padding;
    }

    pub fn append_to_freebox<C>(
        &mut self,
        freebox: &mut FreeBox,
        base_loc: Location,
        component_set: C,
    ) -> Result<(), RenderError>
    where
        C: Fn(LocatedComponent) -> LocatedComponent,
    {
        let mut tree = TaffyTree::new();
        self.register_layout(&mut tree)?;
        // let mut fbox = FreeBox::new(self.style.size());
        for component in &self.contents {
            let layout = tree.layout(component.layout()?)?;
            let loc = Location::new(
                super::freebox::Coordinate::Absolute(layout.location.x as f64),
                super::freebox::Coordinate::Absolute(layout.location.x as f64),
            ) + base_loc.clone();
            let c = LocatedComponent::from_box(component.clone()).location(loc);
            freebox.add(component_set(c));
        }

        Ok(())
    }
}

impl Component for Grid {
    fn render(&self, tree: &TaffyTree) -> Result<Scene, RenderError> {
        let mut scene = Scene::new();

        for component in &self.contents {
            let layout = tree.layout(component.layout()?)?;
            scene.append(
                &component.render(tree)?,
                Some(Affine::translate((
                    layout.location.x as f64,
                    layout.location.y as f64,
                ))),
            );
        }

        Ok(scene)
    }

    fn size(&self) -> Result<Size, RenderError> {
        Ok(self.style.size())
    }

    fn register_layout(&mut self, tree: &mut TaffyTree) -> Result<NodeId, RenderError> {
        if let Some(id) = self.id {
            return Ok(id);
        }
        let ids = self
            .contents
            .iter_mut()
            .map(|a| a.register_layout(tree))
            .collect::<Result<Vec<NodeId>, RenderError>>()?;

        let mut parent_style: taffy::Style = taffy::Style {
            display: Display::Grid,
            grid_template_columns: vec![fr(1f32); self.cols],
            size: taffy::Size {
                width: length(self.size()?.width as f32),
                height: length(self.size()?.height as f32),
            },
            gap: taffy::Size {
                width: length(self.col_gap as f32),
                height: length(self.row_gap as f32),
            },
            ..Default::default()
        };

        if let Some(r) = self.rows {
            parent_style.grid_template_rows = vec![fr(1f32); r];
        }

        match self.style.arrangement() {
            Arrangement::Start => parent_style.justify_content = Some(JustifyContent::START),
            Arrangement::End => parent_style.justify_content = Some(JustifyContent::END),
            Arrangement::Center => parent_style.justify_content = Some(JustifyContent::CENTER),
            Arrangement::SpaceBetween => {
                parent_style.justify_content = Some(JustifyContent::SPACE_BETWEEN)
            }
            Arrangement::SpaceAround => {
                parent_style.justify_content = Some(JustifyContent::SPACE_AROUND)
            }
            Arrangement::SpaceEvenly => {
                parent_style.justify_content = Some(JustifyContent::SPACE_EVENLY)
            }
            Arrangement::SpacedBy(pixels) => {
                parent_style.justify_content = Some(JustifyContent::START);
                if self.row_gap == 0.0 {
                    parent_style.gap.height = length(pixels.clone() as f32);
                }
                if self.col_gap == 0.0 {
                    parent_style.gap.width = length(pixels.clone() as f32);
                }
            }
        }

        match self.style.alignment() {
            Alignment::Start => parent_style.align_items = Some(AlignItems::START),
            Alignment::End => parent_style.align_items = Some(AlignItems::END),
            Alignment::Center => parent_style.align_items = Some(AlignItems::CENTER),
        }

        parent_style.margin = self.style.padding().to_rect();

        let id = tree.new_with_children(parent_style, &ids)?;
        tree.compute_layout(id, taffy::Size::max_content())?;
        self.id = Some(id);
        Ok(id)
    }

    fn layout(&self) -> Result<NodeId, RenderError> {
        match self.id {
            Some(v) => Ok(v),
            None => Err(RenderError::NotComputed),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
