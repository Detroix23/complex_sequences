# Lesson 1: Basic Interactivity & State Management

## The Challenge: "Where do I store my variables?"

In standard Object-Oriented GUI frameworks (like Qt or Java Swing), you create a `Slider` object and it sits in memory. You query it when you need the value.

In **Immediate Mode GUI (ImGui)**, there are no widget objects. You call a function like `ui.slider(...)` *every single frame* (60 times a second).
- If the user isn't touching it, it draws the slider.
- If the user drags it, it draws the slider *and* updates the variable you passed to it.

## The Rust Concept: Closures

To make this work in Rust, we use **Closures**. You mentioned you are new to them.

Think of a closure as a function that carries a backpack of data with it.
1. We define our variables (state) *before* starting the application loop.
2. We "move" these variables into the closure.
3. The closure is called every frame, and because it "owns" those variables, they persist.

```rust
// 1. Define State
let mut zoom = 1.0;

// 2. Define Closure (The "backpack" now contains 'zoom')
let run_ui = move |_, ui| {
    // 3. Update State (Called every frame)
    ui.text("Hello");
    // This slider modifies 'zoom' directly inside the backpack
    ui.slider("Zoom Level", &mut zoom, 0.1, 10.0).build();
};
```

## The Code Structure

In `01_basic_interactivity.rs`, we will build a settings panel with:
- **Fractal Type:** A radio button selection.
- **Iterations:** An integer slider.
- **Zoom/Pan:** Float sliders.
- **Color:** A color picker.

### Instructions
1. Read the code in `01_basic_interactivity.rs`.
2. Copy the content of `01_basic_interactivity.rs` into `src/main.rs`.
3. Run `cargo run`.
