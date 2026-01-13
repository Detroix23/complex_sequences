//! # Complex sequences.
//! src/main.rs  

mod support;
mod gui;
mod fractals;

/// Basic interactivity entry point.
fn main() {
    println!("# Complex sequences.");

    gui::defaults::launch_default();

    eprintln!("(?) main::main() End !");
}
