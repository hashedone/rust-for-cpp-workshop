# Rust workshop agenda

## Part 1 - basics

* Environment setup, warm up (30min)
* Introdudtion to Rust (30min presentation + 30min practice)
    * Variables (declaration, mutability, assignment)
    * Basic function declaration
    * Basic control flow:
        * `if cond` statement
        * `while` loop
        * `loop` loop
        * `for .. in ..` basic loop
    * `println!` and `format!` macros
    * `cargo`
        * `cargo build`, `cargo run`, `cargo clippy`, `cargo fmt`
    * Practice:
        * Fibonacci numbers?
* Module system and typesystem basics (15min presentation + 45min practice)
    * Using standard library and documentation
    * `struct`
    * `enum` as a sum type
    * `trait` as an interface
    * tuples, tuple `structs`
    * basic `match` statement
    * `match`, `if`, `loop`, and scopes as expression
    * Practice:
        * ???
* More advanced typesystem (15min presentation + 45min practice)
    * `enum` as an algebraic type
    * functions generization over `trait`
    * pattern matching and `match` expression
    * `if let`, `while let` expressions
    * error handling, `Option` and `Result`, `?` operator
    * Practice:
        * ???
* Documentation, testing (10min presentation + 20min practice)
    * Documentation comments
    * `cargo doc`
    * Unit tests
    * `cargo test`
    * Documentation test
    * Practice:
        * Document and test previous examples

## Part 2 - semi-advanced

* Part 1 recap, warm up (30min)
* Module system (15min presentation + 45min practice)
    * Using dependencies
    * Creating own modules
    * Practice
         * ???
* More on typesystem and traits (15min presentation + 45min practice)
    * `Iterator`
    * derive
        * `Debug`
        * `Clone` and `Copy`
        * `Eq`, `PartialEq`, `Hash`
    * generic types
    * generic traits vs embeded types
    * borrowing basic
    * Practice
        * ???
* Lifetimes (15min presentation + 45min practice)
    * Borrow checker idea
    * Generalization over lifetimes
    * `Cow`
    * Dealing with lifetime errors
    * Practice
        * More on solving lifetime compiler errors than writing own program
* Closeup session (50min)
    * Covering topics which was asked from audience


## Additional
Those topics are not ment to be covered, but if somehow audience would go really fast, they are planned to be prepared an may be covered
* Rust nightly
* HRTBs (higher rank trait bounds)
* Threading basics
* Unsafe, soundness, FFI
* `macro_rules!`
