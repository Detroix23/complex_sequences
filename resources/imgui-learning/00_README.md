# Imgui Learning Path

This directory contains a step-by-step guide to mastering `imgui-rs` for your fractal project.

## Roadmap

### Lesson 1: Basic Interactivity & Settings
**Goal:** Build the control panel for the fractal explorer.
**Concepts:**
- **Closures:** How to maintain application state (zoom, position, colors) across frames.
- **Immediate Mode:** Understanding that widgets are function calls, not objects.
- **Widgets:** Sliders, Color Pickers, and Buttons.

**Files:**
- `01_basic_interactivity.md`: Detailed explanation.
- `01_basic_interactivity.rs`: Runnable code.

### Lesson 2: Rendering to Texture (Next Step)
**Goal:** Display a dynamic image (the fractal) inside an ImGui window.
**Concepts:**
- **Glium Textures:** Creating and updating pixel data on the GPU.
- **Texture ID:** Passing GPU textures to ImGui.
- **Pixel Buffer:** Managing the raw data array for our fractal.
