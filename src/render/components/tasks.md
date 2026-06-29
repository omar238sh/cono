# Component Restructuring Task List

This document lists the concrete, actionable tasks required to refactor the rendering component architecture based on the builder, wrapper, and layout-decoupling models.

---

## Task 1: Fluent Builder Refactoring

Convert existing mutable config setters into a chained builder API.

### 1.1 Refactor `Column`
- **Target File**: [column.rs](file:///home/omar/Desktop/cono/src/render/components/column.rs)
- **Modifications**:
  - Replace `edit_arrangement(&mut self, arrangement: Arrangement)` with `arrangement(mut self, arrangement: Arrangement) -> Self`.
  - Replace `edit_alignment(&mut self, alignment: Alignment)` with `alignment(mut self, alignment: Alignment) -> Self`.
  - Replace `edit_padding(&mut self, padding: Padding)` with `padding(mut self, padding: Padding) -> Self`.
  - Update `push` to support chained item addition, e.g., `pub fn with_child<T: Component + 'static>(mut self, child: T) -> Self`.

### 1.2 Refactor `SvgBox`
- **Target File**: [svg.rs](file:///home/omar/Desktop/cono/src/render/components/svg.rs)
- **Modifications**:
  - Replace `edit_size(&mut self, new_size: Size)` with `size(mut self, new_size: Size) -> Self`.
  - Replace `scale(&mut self, factor: f64)` with `scaled(mut self, factor: f64) -> Self`.

---

## Task 2: Component Decorators (Composition Model)

Create wrapper components that implement `Component` to handle visual styles generically, keeping leaf and layout nodes focused.

### 2.1 Implement `Container<C>` Wrapper
- **Target File**: Create `src/render/components/container.rs` (register in [mod.rs](file:///home/omar/Desktop/cono/src/render/components/mod.rs)).
- **Structure**:
  ```rust
  pub struct Container<C: Component> {
      pub child: C,
      pub background_color: Option<Color>,
      pub border_color: Option<Color>,
      pub border_width: f64,
      pub border_radius: f64,
      pub padding: Padding,
  }
  ```
- **Tasks**:
  - Implement `Component` trait for `Container<C>`.
  - In `render()`, compute new bounding boxes considering padding/borders, draw the background and borders using `Vello`, and append the child scene translated by padding.
  - Implement a fluent builder API for `Container<C>`.

### 2.2 Implement `Opacity<C>` Wrapper
- **Target File**: Create `src/render/components/opacity.rs` (register in [mod.rs](file:///home/omar/Desktop/cono/src/render/components/mod.rs)).
- **Structure**:
  ```rust
  pub struct Opacity<C: Component> {
      pub child: C,
      pub alpha: f32,
  }
  ```
- **Tasks**:
  - Implement `Component` trait for `Opacity<C>`.
  - In `render()`, apply opacity transformations or use `Vello` scene blending layer.

---

## Task 3: Decouple Layout Calculations from Vello Scene Creation

Standardize component interaction with the `taffy` layout engine without mixing it directly into the scene builder.

### 3.1 Define the `LayoutNode` Trait
- **Target File**: [frame.rs](file:///home/omar/Desktop/cono/src/render/frame.rs)
- **Structure**:
  ```rust
  pub trait LayoutNode {
      fn register_layout(&self, tree: &mut TaffyTree) -> Result<NodeId, TaffyError>;
  }
  ```
- **Tasks**:
  - Make `Component` inherit from or compose with `LayoutNode`.
  - Migrate internal taffy-specific layout calculations (e.g. from `Column::compute_layout`) into the `LayoutNode::register_layout` implementation.

---

## Task 4: Layout Validation (Post-Layout Checks)

Utilize computed node structures to assert constraints or perform bounds validation.

### 4.1 Refactor `Validate` Trait
- **Target File**: [validate.rs](file:///home/omar/Desktop/cono/src/render/components/validate.rs)
- **Structure**:
  ```rust
  pub trait Validate: Component {
      fn validate(&self, tree: &TaffyTree, root: NodeId) -> Result<(), LayoutValidationError>;
  }
  ```
- **Tasks**:
  - Define custom errors in `error.rs` for layout overflow or empty sizes.
  - Assert that computed child sizes fit within parent constraints.
