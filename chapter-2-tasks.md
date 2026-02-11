# Chapter 2 Tasks: Programming a Guessing Game

This chapter focuses on learning fundamental programming concepts through building a guessing game, comparing Python and Rust approaches, and integrating them via PyO3.

## Overview

Chapter 2 introduces:
- **Variables and Mutability** - How to declare and change data
- **Data Types** - Integers, floats, booleans, characters, and strings
- **Functions** - Defining and calling functions with parameters and return values
- **Comments** - Documenting code
- **Control Flow** - Conditional logic and loops

The chapter project builds an interactive number guessing game.

---

## Part 1: Rust Concepts Review

### Task 1.1: Variables and Mutability
**Objective:** Understand Rust's approach to variables and mutability

**Steps:**
1. Read The Rust Book section: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
2. Learn about:
   - The `let` keyword for variable declarations
   - Why Rust variables are immutable by default
   - Using `mut` keyword to make variables mutable
   - Variable shadowing vs. mutability
3. Write example code demonstrating:
   - Declaring an immutable variable
   - Attempting to modify an immutable variable (observe the error)
   - Declaring a mutable variable and modifying it
   - Shadowing a variable with the same name
   - Shadowing to change a variable's type

**Key Questions to Consider:**
- Why does Rust prefer immutability by default?
- What's the difference between `mut` and shadowing?
- When would you choose one over the other?

**Documentation:**
- Rust Book: Chapter 3.1 - Variables and Mutability
- Rust Reference: https://doc.rust-lang.org/reference/variables.html

---

### Task 1.2: Data Types
**Objective:** Learn Rust's type system and common data types

**Steps:**
1. Read The Rust Book section: https://doc.rust-lang.org/book/ch03-02-data-types.html
2. Learn about Rust's scalar types:
   - Integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
   - Floating-point: `f32`, `f64`
   - Booleans: `bool`
   - Characters: `char`
3. Learn about Rust's compound types:
   - Tuples
   - Arrays
4. Write example code demonstrating:
   - Declaring variables with explicit types
   - Integer literals in different formats (decimal, hex, octal, binary)
   - Floating-point operations
   - Boolean expressions
   - Working with characters
   - Creating and accessing tuples
   - Creating and accessing arrays
   - Understanding array bounds

**Key Questions to Consider:**
- What's the difference between `i32` and `u32`?
- When would you use an array vs. a tuple?
- What happens when you access an array out of bounds?

**Documentation:**
- Rust Book: Chapter 3.2 - Data Types
- Rust Reference: https://doc.rust-lang.org/reference/types.html

---

### Task 1.3: Functions
**Objective:** Master function definitions and usage in Rust

**Steps:**
1. Read The Rust Book section: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
2. Learn about:
   - Function syntax with `fn` keyword
   - Parameters and type annotations
   - Return values and the `->` syntax
   - Expressions vs. statements
   - Implicit return (no semicolon)
3. Write example code demonstrating:
   - Functions with no parameters
   - Functions with multiple parameters
   - Functions that return values (with and without `return`)
   - The difference between statements and expressions
   - Using expressions as function bodies

**Key Questions to Consider:**
- Why must function parameters have type annotations?
- What's the difference between a statement and an expression in Rust?
- Why does the last expression in a function body not need a semicolon?

**Documentation:**
- Rust Book: Chapter 3.3 - Functions
- Rust Reference: https://doc.rust-lang.org/reference/items/functions.html

---

### Task 1.4: Comments
**Objective:** Learn documentation and comment conventions in Rust

**Steps:**
1. Read The Rust Book section: https://doc.rust-lang.org/book/ch03-04-comments.html
2. Learn about:
   - Single-line comments with `//`
   - Multi-line comments
   - Documentation comments `///` and `//!`
   - Best practices for commenting code
3. Write example code demonstrating:
   - Single-line comments
   - Block comments
   - Documentation comments for functions
   - Module-level documentation

**Key Questions to Consider:**
- When should you use documentation comments vs. regular comments?
- What is the purpose of `//!` comments?

**Documentation:**
- Rust Book: Chapter 3.4 - Comments
- Rustdoc: https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html

---

### Task 1.5: Control Flow
**Objective:** Master conditional logic and loops in Rust

**Steps:**
1. Read The Rust Book section: https://doc.rust-lang.org/book/ch03-05-control-flow.html
2. Learn about:
   - `if` expressions
   - `else if` chains
   - Using `if` as an expression
   - `loop` infinite loops and breaking
   - `while` loops
   - `for` loops with ranges
3. Write example code demonstrating:
   - Basic `if/else` statements
   - `if` used as expressions (assigning values)
   - Multiple `else if` conditions
   - Using `loop` with `break` and `continue`
   - Using `while` loops
   - Using `for` loops with ranges (e.g., `1..10`)
   - Nested control flow

**Key Questions to Consider:**
- How is Rust's `if` different from Python's `if`?
- What's the advantage of using `for` loops with ranges?
- When would you choose `loop`, `while`, or `for`?

**Documentation:**
- Rust Book: Chapter 3.5 - Control Flow
- Rust Reference: https://doc.rust-lang.org/reference/expressions/if-expr.html

---

### Task 1.6: External Crates
**Objective:** Learn how to use external dependencies in Rust

**Steps:**
1. Read about Cargo.toml dependencies: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
2. Learn about:
   - Adding dependencies to `Cargo.toml`
   - Using `use` to bring items into scope
   - The `rand` crate for generating random numbers
3. Write example code demonstrating:
   - Adding `rand` to `Cargo.toml`
   - Importing and using the `rand` crate
   - Generating random numbers within a range

**Documentation:**
- Cargo Book: Specifying Dependencies
- `rand` crate documentation: https://docs.rs/rand/

---

### Task 1.7: Standard Library I/O
**Objective:** Learn to read user input in Rust

**Steps:**
1. Read about `std::io`: https://doc.rust-lang.org/std/io/index.html
2. Learn about:
   - The `std::io` module
   - Using `stdin()` and `read_line()`
   - Handling errors with `expect()` and `unwrap()`
3. Write example code demonstrating:
   - Reading a line of user input
   - Understanding the `Result` type
   - Handling potential errors

**Documentation:**
- Rust Book: Chapter 2 (Guessing Game example)
- std::io documentation: https://doc.rust-lang.org/std/io/

---

## Part 2: Python Equivalent Concepts

### Task 2.1: Variables in Python
**Objective:** Review Python's approach to variables

**Steps:**
1. Review Python variable rules:
   - No declaration keyword needed
   - Variables are always mutable
   - Dynamic typing
2. Write example code demonstrating:
   - Variable assignment and reassignment
   - Type changes (e.g., `x = 5; x = "hello"`)
   - Variable naming rules
3. Compare with Rust:
   - What's the key difference between Python and Rust variables?
   - Why doesn't Python need type annotations?

**Documentation:**
- Python Tutorial: https://docs.python.org/3/tutorial/introduction.html#using-python-as-a-calculator

---

### Task 2.2: Python Data Types
**Objective:** Review Python's type system

**Steps:**
1. Review Python's built-in types:
   - Integers (arbitrary precision)
   - Floats
   - Booleans
   - Strings
   - Lists (similar to arrays but dynamic)
   - Tuples
   - Dictionaries (similar to HashMaps)
2. Write example code demonstrating:
   - Working with integers and floats
   - Type checking with `type()`
   - Using lists and tuples
   - Understanding type conversions
3. Compare with Rust:
   - How does Python's dynamic typing differ from Rust's static typing?
   - What's the difference between Python lists and Rust arrays?

**Documentation:**
- Python Documentation: https://docs.python.org/3/library/stdtypes.html

---

### Task 2.3: Python Functions
**Objective:** Review Python functions

**Steps:**
1. Review Python function syntax:
   - `def` keyword
   - No type annotations required (but optional with type hints)
   - Return statements
   - Default parameters
2. Write example code demonstrating:
   - Functions with and without parameters
   - Functions with return values
   - Functions that return `None` implicitly
   - Using type hints (Python 3.5+)
3. Compare with Rust:
   - How are Python and Rust function signatures different?
   - What's the advantage of Rust's required type annotations?

**Documentation:**
- Python Tutorial: https://docs.python.org/3/tutorial/controlflow.html#defining-functions

---

### Task 2.4: Python Control Flow
**Objective:** Review Python's conditional and loop constructs

**Steps:**
1. Review Python control flow:
   - `if/elif/else` statements
   - `while` loops
   - `for` loops with iterables
   - `break` and `continue`
2. Write example code demonstrating:
   - Conditional logic with `if/elif/else`
   - Iterating over lists with `for`
   - Using `range()` in `for` loops
   - Using `while` loops
   - Breaking and continuing
3. Compare with Rust:
   - How does Python's `elif` compare to Rust's `else if`?
   - What's the difference between Python's `for` and Rust's `for` with ranges?

**Documentation:**
- Python Tutorial: https://docs.python.org/3/tutorial/controlflow.html

---

### Task 2.5: Python Input and Output
**Objective:** Review Python's I/O capabilities

**Steps:**
1. Review Python I/O:
   - `print()` function
   - `input()` function
   - f-strings for formatting (Python 3.6+)
2. Write example code demonstrating:
   - Reading user input with `input()`
   - Converting input to different types
   - Using f-strings for output
   - Error handling for invalid input
3. Compare with Rust:
   - How is `input()` in Python different from `read_line()` in Rust?
   - What error handling differences do you notice?

**Documentation:**
- Python Tutorial: https://docs.python.org/3/tutorial/inputoutput.html

---

## Part 3: PyO3 Integration Exercise

### Task 3.1: PyO3 Number Generation
**Objective:** Create a Rust function to generate random numbers callable from Python

**Concepts to Practice:**
- Using the `rand` crate in Rust
- Exposing functions to Python with `#[pyfunction]`
- Type conversion between Python and Rust
- Parameter passing and return values

**Steps:**
1. Read the PyO3 guide on functions: https://pyo3.rs/latest/function.html
2. Understand how to:
   - Add `rand` as a dependency in `Cargo.toml`
   - Create a function that generates a random number within a range
   - Expose it to Python with the correct types
3. Implement a Rust function with the following signature:
   - Function name: `generate_random_number(min: u32, max: u32) -> u32`
   - Should generate a random number between `min` and `max` (inclusive)
   - Should be callable from Python

**Documentation:**
- PyO3 Guide: Function Signatures
- `rand` crate docs: https://docs.rs/rand/

---

### Task 3.2: PyO3 Input Validation
**Objective:** Create Rust functions to validate user input

**Concepts to Practice:**
- String handling in Rust
- Boolean returns
- Error handling concepts
- Type conversions

**Steps:**
1. Implement Rust functions for input validation:
   - `is_valid_guess(guess: &str, min: u32, max: u32) -> bool`
     - Should check if the string can be parsed as a number
     - Should check if the number is within the valid range
   - `parse_guess(guess: &str) -> Option<u32>`
     - Should attempt to parse the string as a number
     - Should return `Some(number)` if successful, `None` if not
2. Make these functions callable from Python

**Documentation:**
- PyO3 Guide: String Handling
- Rust Book: Option type (Chapter 6, but useful here)

---

### Task 3.3: PyO3 Game Logic
**Objective:** Create the core game logic in Rust

**Concepts to Practice:**
- Multiple functions working together
- Comparison operations
- Returning structured information

**Steps:**
1. Implement a function to compare the guess:
   - `check_guess(secret: u32, guess: u32) -> String`
     - Returns "Too high!" if guess > secret
     - Returns "Too low!" if guess < secret
     - Returns "Correct!" if guess == secret
2. Test calling this function from Python

**Documentation:**
- PyO3 Guide: Conversions

---

## Part 4: Chapter Project - Enhanced Guessing Game

### Task 4.1: Project Design
**Objective:** Plan the guessing game project

**Design Requirements:**
- The game should generate a random number between 1 and 100
- The user should be able to guess the number
- The game should provide feedback (too high, too low, correct)
- The game should track the number of attempts
- The game should allow the user to play multiple rounds
- Use Rust for random number generation and game logic
- Use Python for user interaction and game flow

**Architecture:**
- **Rust (lib.rs):**
  - `generate_random_number(min, max)` - Generate the secret number
  - `check_guess(secret, guess)` - Compare and provide feedback
  - `is_valid_guess(guess_str, min, max)` - Validate user input
  - `parse_guess(guess_str)` - Convert string to number

- **Python (main.py):**
  - Main game loop
  - User input and output
  - Game state tracking
  - Multiple round support

---

### Task 4.2: Implement Rust Game Logic
**Objective:** Build the Rust backend

**Implementation Steps:**
1. Add `rand` dependency to `Cargo.toml`
2. Implement all Rust functions from Task 4.1
3. Expose functions using `#[pyfunction]`
4. Update the module to export all functions
5. Test building with `maturin develop`

**Tips:**
- Remember to handle the `rand` crate's setup
- Use appropriate error handling for parsing
- Consider edge cases (empty strings, non-numeric input)

---

### Task 4.3: Implement Python Interface
**Objective:** Build the Python frontend

**Implementation Steps:**
1. Import the Rust functions from your PyO3 module
2. Create the main game loop:
   - Generate a secret number using Rust
   - Loop until the user guesses correctly
   - Read and validate user input using Rust functions
   - Get feedback from Rust comparison function
   - Track and display attempt count
3. Add support for multiple rounds:
   - Ask if the user wants to play again
   - Reset game state for each round
   - Keep statistics (total games played, best score)
4. Add user-friendly messages and instructions

**Tips:**
- Use f-strings for clean output formatting
- Handle invalid input gracefully
- Make the game clear and engaging

---

### Task 4.4: Testing and Refinement
**Objective:** Test the complete game

**Testing Steps:**
1. Test the game with various inputs:
   - Valid guesses (too low, too high, correct)
   - Invalid inputs (letters, symbols, out of range)
   - Edge cases (minimum value, maximum value)
2. Test multiple rounds:
   - Play multiple times in one session
   - Verify statistics are tracked correctly
3. Test error handling:
   - What happens if input is empty?
   - What happens with very large numbers?
4. Refine the user experience:
   - Add helpful messages
   - Improve formatting
   - Consider adding difficulty levels

---

### Task 4.5: Documentation and Cleanup
**Objective:** Polish and document the project

**Steps:**
1. Add docstrings to Python functions
2. Add documentation comments to Rust functions (`///`)
3. Clean up any debug code or comments
4. Update `README.md` with:
   - Project description
   - How to run the game
   - Key features
   - What you learned

---

## Learning Outcomes

After completing Chapter 2, you should be able to:

### Rust
- ✅ Declare and use variables with `let` and `mut`
- ✅ Understand and use Rust's data types
- ✅ Define and call functions with proper type annotations
- ✅ Write clear comments and documentation
- ✅ Use `if`, `else if`, `loop`, `while`, and `for` control flow
- ✅ Add and use external crates (like `rand`)
- ✅ Read user input using `std::io`
- ✅ Handle basic errors with `expect()` and `unwrap()`

### Python
- ✅ Understand the differences between Python and Rust variable systems
- ✅ Compare Python's dynamic typing with Rust's static typing
- ✅ Use type hints in Python functions
- ✅ Compare control flow structures between languages

### PyO3 Integration
- ✅ Pass integers and strings between Python and Rust
- ✅ Expose multiple Rust functions to Python
- ✅ Build a complete application using both languages
- ✅ Understand when to use Rust vs. Python for different tasks

---

## Additional Resources

### Rust Resources
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rust Standard Library: https://doc.rust-lang.org/std/

### Python Resources
- Python Tutorial: https://docs.python.org/3/tutorial/
- Python Standard Library: https://docs.python.org/3/library/

### PyO3 Resources
- PyO3 User Guide: https://pyo3.rs/
- PyO3 API Documentation: https://docs.rs/pyo3/

---

## Next Steps

After completing Chapter 2:
1. Update `memory-bank/concepts-mastered.md` with new concepts
2. Update `memory-bank/learning-progress.md` to mark Chapter 2 complete
3. Add your project to `memory-bank/projects-completed.md`
4. Move on to Chapter 3: Common Programming Concepts

---

*Created: 2026-02-08*