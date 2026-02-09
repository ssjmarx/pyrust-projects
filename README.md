# Python-Rust Learning Project

A learning workspace for mastering Python and Rust together with PyO3 integration.

## Project Structure

```
pyrust-projects/
├── memory-bank/
│   ├── concepts-mastered.md    # Track concepts you've learned
│   ├── learning-progress.md    # Progress through The Rust Book
│   └── projects-completed.md   # Completed projects
├── rust/
│   ├── Cargo.toml              # Rust project config with PyO3
│   └── src/
│       └── lib.rs              # Rust functions exposed to Python
├── venv/                       # Python virtual environment
├── main.py                     # Python entry point for testing
├── .clinerules                 # AI agent guidelines (DO NOT WRITE CODE)
└── README.md                   # This file
```

## Getting Started

### 1. Activate the Python Virtual Environment

```bash
source venv/bin/activate
```

### 2. Build the Rust Extension

```bash
cd rust
maturin develop
cd ..
```

### 3. Test from Python

```bash
python main.py
```

## Learning Approach

### Concept Learning
For each concept from The Rust Book:
1. Implement a Python example demonstrating the concept
2. Implement a Rust equivalent
3. Create a PyO3 function to bridge both languages
4. Test integration between Python and Rust

### Chapter Projects
For each chapter of The Rust Book:
1. Review Rust concepts from the chapter
2. Identify equivalent Python concepts
3. Create a project using both languages via PyO3
4. Document what you learned

## Recommended Resources

### Primary Learning Materials
- **The Rust Book**: https://doc.rust-lang.org/book/
- **PyO3 Guide**: https://pyo3.rs/latest/
- **Python Documentation**: https://docs.python.org/

### PyO3 Specific Topics
- Functions: https://pyo3.rs/latest/function.html
- Python Types: https://pyo3.rs/latest/types.html
- Error Handling: https://pyo3.rs/latest/error_handling.html
- Classes: https://pyo3.rs/latest/class.html
- Module Structure: https://pyo3.rs/latest/module.html

## Development Workflow

1. **Choose a concept** from the current Rust Book chapter
2. **Read the documentation** for both Python and Rust implementations
3. **Implement** the concept in both languages
4. **Create a PyO3 bridge** to connect them
5. **Test** the integration thoroughly
6. **Document** what you learned in the memory-bank files
7. **Ask for feedback** on your implementation

## Example Usage

### In Python (main.py)
```python
import rust

# Call a Rust function from Python
rust.say_hello()
```

### In Rust (rust/src/lib.rs)
```rust
#[pyo3::pymodule]
mod rust {
  use pyo3::prelude::*;

  #[pyfunction]
  fn say_hello() {
    println!("Hello!  This is from rust.")
  }
}
```

This is the modern PyO3 module syntax using nested modules, which is the recommended approach from the official PyO3 documentation.

## Notes

- PyO3 is a Rust crate (installed via Cargo), not a Python package
- The `.clinerules` file instructs AI agents to guide, not code for you
- Focus on understanding concepts, not just getting working code
- Progress tracking is in the `memory-bank/` directory

## Quick Reference

- Activate venv: `source venv/bin/activate`
- Build with maturin: `cd rust && maturin develop && cd ..`
- Run Python: `python main.py`
- Install maturin: `pip install maturin` (if not already installed)