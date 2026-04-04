# Lesson 1: Basic Interactivity & State Management

## The Challenge: "Where do I store my variables?"

In **Immediate Mode GUI (ImGui)**, there are no widget objects. You call a function like `ui.slider(...)` *every single frame*.

## The Rust Concept: Closures & Explicit Namespacing

To make this work in Rust, we use **Closures** with the `move` keyword. This allows the closure to "own" the state variables and keep them alive across frames.

### Best Practice: Explicit Namespacing
In this project, we avoid wildcard imports or bringing many symbols into the top-level scope. Instead, we use **explicit namespacing**.
- **Don't:** `use imgui::*;` then `Ui`.
- **Do:** Use `imgui::Ui`, `imgui::Condition`, etc.

This makes it very clear which library is providing each type or constant.

### Explicit Closure Annotations
When defining the `run_ui` closure, we explicitly annotate the `ui` parameter to ensure type safety and clarity:

```rust
move |_, ui: &mut imgui::Ui| {
    // UI logic here...
}
```

## The Code Structure

In `01_basic_interactivity.rs`, we build a settings panel with:
- **Fractal Type:** Radio buttons.
- **Iterations:** Integer slider.
- **Zoom/Pan:** Float sliders and Drag scalars.
- **Color:** A color picker using `imgui::ColorEdit`.

### Instructions
1. Read the code in `01_basic_interactivity.rs`.
2. Copy its content into `src/main.rs`.
3. Run `cargo run` (or `cargo check` to verify).