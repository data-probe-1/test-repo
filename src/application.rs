//! The `Application` type and its behavior.
//!
//! Rust has no classes. The closest equivalent is a `struct` (the data) plus an
//! `impl` block (the methods). Each file is a *module*, and its name (`application`)
//! comes from the file name.

/// The application. It has no fields yet, so it is a "unit struct".
pub struct Application;

impl Application {
    /// Creates a new `Application`.
    ///
    /// Rust has no constructors. By convention, an associated function named `new`
    /// does that job. It has no `self` parameter, like a static method in C++.
    pub fn new() -> Self {
        Application
    }

    /// Runs the application: prints "Hello World!" to the terminal.
    ///
    /// `&self` makes this a method. It borrows the instance read-only, much like a
    /// `const` member function in C++.
    pub fn run(&self) {
        println!("Hello World!");
    }
}
