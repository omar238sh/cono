# Adding Lua Support to Components

This guide outlines the architecture and implementation steps to add Lua scripting support to the component system using the `mlua` crate (already configured in `Cargo.toml`).

---

## 1. Overview
The goal is to allow users to define component trees and layouts using Lua scripts. 
For example, a Lua script should be able to do:
```lua
local col = cono.Column(1080, 1920)
col:push(cono.Text("Hello from Lua"):color("#ff0000"):size(24))
return col
```

To achieve this, we need to:
1. Implement `mlua::UserData` for our Rust components (`Text`, `Column`, `Row`, `FreeBox`, `Container`).
2. Register constructor functions in the Lua global context.
3. Handle component ownership transfers from Lua to Rust.

---

## 2. Step-by-Step Implementation

### Step 2.1: Implement `mlua::UserData` for Components
For each component that should be accessible in Lua, implement the `mlua::UserData` trait.
In the implementation:
- **Methods**: Map the Rust builder methods (e.g., `color`, `size`, `push`) to Lua methods.
- **Cloning/Passing**: Since Rust components will be nested inside parents (e.g., pushing a `Text` into a `Column`), we need to manage ownership. Since Lua values are GC-managed, we can wrap our components or pass them by taking ownership.

> [!NOTE]
> Since `mlua::UserData` methods take `&mut self`, we can implement method chaining by returning the object back to Lua, or mutating it in-place and returning `self`.

### Step 2.2: Implement Component Ownership Transfer
When components are pushed into containers (e.g. `column:push(child)`), Lua transfers ownership of that component to the container.
- To handle this in Rust, the `UserData` wrapper for components can store the component inside an `Option<C>`.
- When `push` is called, the parent component can `take()` the inner component from the child wrapper.
- Alternatively, components can be wrapped in `Rc<RefCell<dyn Component>>` or similar shared ownership pointers to allow referencing them from both Lua and Rust.

### Step 2.3: Register Constructors in Lua Context
Create a registration function (e.g., `register_cono_module(lua: &Lua)`) to expose constructors:
- Create a Lua table `cono`.
- Add functions to the table: `cono.Text(content)`, `cono.Column(width, height)`, `cono.Row(width, height)`, `cono.FreeBox(width, height)`.
- Set `cono` as a global variable in the Lua state.

### Step 2.4: Execute Lua Script and Retrieve Root Component
In the main runner or renderer:
1. Instantiate `mlua::Lua`.
2. Register the component module.
3. Load and execute the Lua script.
4. Retrieve the returned root component (which must implement `Component`).
5. Run layout calculations and render it using the standard pipeline.
