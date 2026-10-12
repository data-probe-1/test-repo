//! Hello World: a minimal Rust program for beginners.
//!
//! Build and run with `cargo run`.

// Declares the `application` module. Rust loads it from `src/application.rs`.
mod application;

// Brings `Application` into scope, so we can write `Application`
// instead of `application::Application`.
use application::Application;

/// Program entry point. Every Rust executable starts at `main`.
fn main() {
    let application = Application::new();
    application.run();
}
