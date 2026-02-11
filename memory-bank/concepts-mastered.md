# Concepts Mastered

Track Python and Rust concepts that you have successfully learned and can implement independently.

## Python Concepts

- **Python Script Structure** - No required main function, uses `if __name__ == "__main__":` guard
- **`print()` Function** - Built-in function for output to console
- **`input()` Function** - Reading user input from stdin
- **Python `match` Statement** - Pattern matching (Python 3.10+) for conditional logic
- **Recursive Functions** - Functions that call themselves
- **Function Return Statements** - Explicit return values, implicit `None` without return
- **Virtual Environments** - Isolated Python environments with `venv`
- **`requirements.txt`** - Tracking Python dependencies

## Rust Concepts

- **`main()` Function** - Entry point of Rust programs
- **`println!` Macro** - Printing to console with formatting
- **`format!` Macro** - String formatting and creation
- **Cargo** - Rust's package manager and build system
- **`cargo run`** - Compile and execute in one command
- **`cargo check`** - Fast syntax checking without building
- **`cargo build --release`** - Optimized release builds
- **Cargo.toml** - Project metadata and dependency configuration
- **`crate-type = ["cdylib"]`** - C-style dynamic library for Python extensions
- **Rust `match` Expression** - Pattern matching for conditional logic
- **String Creation** - `String::from()`, `.to_string()`, and `format!()` comparison
- **Function Syntax** - `fn` keyword, parameters, return types, semicolons

## PyO3 Integration Concepts

- **Maturin** - Build tool and packager for Python extensions written in Rust
- **Maturin `develop`** - Editable installs that rebuild on changes
- **Modern PyO3 Module Syntax** - Using `#[pyo3::pymodule]` with nested modules
- **pyo3::prelude::\*** - Importing common PyO3 types and macros
- **`#[pyfunction]`** - Macro to expose Rust functions to Python
- **Type Conversion** - Python `str` ↔ Rust `&str` automatic conversion
- **Parameter Passing** - Passing Python values to Rust functions
- **Return Values** - Returning Rust `String` to Python
- **cdylib crate type** - Creating C-style dynamic libraries for Python
- **Editable Installs** - Virtual environment pointers to source code

---
*Last updated: 2026-02-08 - Chapter 1 concepts added!*
