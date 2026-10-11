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

    /// Counts from `a` to `b` inclusive, printing each number on its own line.
    ///
    /// `a..=b` is an *inclusive range*: it includes `b`. The plain form `a..b`
    /// would stop one short, like `for (int i = a; i < b; i++)` in C++. If `a`
    /// is greater than `b`, the range is empty and nothing is printed.
    pub fn count(&self, a: i32, b: i32) {
        for number in a..=b {
            println!("{number}");
        }
    }

    /// Adds `a` and `b` and returns the result.
    ///
    /// `-> i32` is the return type. The last expression in a function is the return
    /// value, so no `return` keyword is needed.
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
