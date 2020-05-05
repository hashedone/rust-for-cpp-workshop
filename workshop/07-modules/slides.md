# Modules and hermetization

* Modules
* Hermetization
* Dependencies
* Tests
* Documentation
* Conditional compilation

---

### Declaring module

```rust
mod shapes {
    struct Circle(f32);
    struct Square(f32);
    struct Rect(f32, f32);
}

fn main() {
    let circle = shapes::Circle(10.0);
}
```

---

### Declaring module

```
error[E0603]: tuple struct constructor `Circle` is private
 --> src/main.rs:9:26
  |
3 |     struct Circle(f32);
  |                   --- a constructor is private if any of the fields is private
...
9 |     let circle = shapes::Circle(10.0);
  |                          ^^^^^^ this tuple struct constructor is private
  |
note: the tuple struct constructor `Circle` is defined here
 --> src/main.rs:3:5
  |
3 |     struct Circle(f32);
  |     ^^^^^^^^^^^^^^^^^^^
```

---

### `pub` Visibility

```rust
mod shapes {
    pub struct Circle(f32);
    pub struct Square(f32);
    pub struct Rect(f32, f32);
}

fn main() {
    let circle = shapes::Circle(10.0);
}
```

---

### Extracting modules to files

```rust
// main.rs
mod shape;

fn main() {
    let circle = shapes::Circle(10.0);
}
```

```rust
// shape.rs
pub struct Circle(f32);
pub struct Square(f32);
pub struct Rect(f32, f32);
```

---

### Going deeper

```rust
// main.rs
mod shape;

fn main() {
    let circle = shapes::circle::Circle();
}
```

```rust
// shape.rs
pub mod circle;
pub mod square;
pub mod rect;
```

```rust
// shape/circle.rs
pub Circle(f32);
```

---

### Old style modules nesting

```rust
// main.rs
mod shape;

fn main() {
    let circle = shapes::circle::Circle();
}
```

```rust
// shape/mod.rs
pub mod circle;
pub mod square;
pub mod rect;
```

```rust
// shape/circle.rs
pub Circle(f32);
```

---

### `super` can be used to access parent module

```rust
pub struct Circle(f32);

mod utils {
    pub fn area(circle: &super::Circle) -> f32 {
        2.0 * std::f32::consts::PI * circle.0
    }
}

fn main() {
    let circle = Circle(4.0);
    println!("Area: {}", utils::area(&circle));
}
```

```
Area: 50.265484
```

---

### `crate` can be used to access root module

```rust
pub struct Circle(f32);

mod utils {
    pub mod inner {
        pub fn area(circle: &crate::Circle) -> f32 {
            2.0 * std::f32::consts::PI * circle.0
        }
    }
}

fn main() {
    let circle = Circle(4.0);
    println!("Area: {}", utils::inner::area(&circle));
}
```


---

### By default items declared in modules are visible in children modules

```rust
struct Circle(f32);

// ...

mod utils {
    pub fn circ_to_area(radius: f32) -> f32 {
        let circle = super::Circle(radius);
        circle.circumference() / circle.area()
    }
}

fn main() {
    println!("Ratio: {}", utils::circ_to_area(5.0));
}
```

```
Ratio: 2.5
```

---

### `pub` keyword before any entity means, that whoever has access to this module, has also access to this entity

```rust
mod shapes {
    mod circle {
        pub struct Circle(f32);

        // ...
    }

    pub fn circ_to_area(radius: f32) -> f32 {
        let circle = circle::Circle(radius);
        circle.circumference() / circle.area()
    }
}
```

---

### `pub` keyword before any entity means, that whoever has access to this module, has also access to this entity
```rust
fn main() {
    println!("Ration: {}", shapes::circ_to_area(7.0));
    // Doesn't compile: `shapes::circle` is private
    // let circle = shapes::circle::Circle(3.0);
}
```

---

### We can use `pub` on fields on structures

```rust
pub struct Circle(pub f32);
pub struct Rectangle {
    pub a: f32,
    pub b: f32,
}
```

---

### `pub(super)` means, that enitity (or field) is visible only by parent module

```rust
mod shapes {
    pub mod circle {
        pub(super) struct Circle(f32);

        // ...
    }

    pub fn circ_to_area(radius: f32) -> f32 {
        let circle = circle::Circle(radius);
        circle.circumference() / circle.area()
    }
}
```

---

### `pub(super)` means, that enitity (or field) is visible only by parent module

```rust
fn main() {
    println!("Ration: {}", shapes::circ_to_area(7.0));
    // Doesn't compile: `shape::circle::Circle` is private
    // outside of `shape::circle`
    // let circle = shapes::circle::Circle(3.0);
}
```

---

### `pub(in path)` is more general way to point whole path in which entity is visible

```rust
mod shapes {
    pub mod circle {
        pub(in crate::shapes) struct Circle(f32);

        // ...
    }

    pub fn circ_to_area(radius: f32) -> f32 {
        let circle = circle::Circle(radius);
        circle.circumference() / circle.area()
    }
}
```

---

### `pub(in path)` is more general way to point whole path in which entity is visible

```rust
fn main() {
    println!("Ration: {}", shapes::circ_to_area(7.0));
    // Doesn't compile: `shape::circle::Circle` is private
    // outside of `shape::circle`
    // let circle = shapes::circle::Circle(3.0);
}
```
---

### `use` is a way to import symbol to scope

```rust
pub struct Circle(f32);

mod utils {
    use super::Circle;
    pub fn area(circle: &Circle) -> f32 {
        2.0 * std::f32::consts::PI * circle.0
    }
}

fn main() {
    let circle = Circle(4.0);
    println!("Area: {}", utils::area(&circle));
}
```

---

### It can be used in function level

```rust
pub struct Circle(f32);

mod utils {
    pub fn area(circle: &Circle) -> f32 {
        use super::Circle;
        2.0 * std::f32::consts::PI * circle.0
    }
}

fn main() {
    let circle = Circle(4.0);
    println!("Area: {}", utils::area(&circle));
}
```

---

### Or even scope level

```rust
pub struct Circle(f32);

mod utils {
    pub fn area(circle: &Circle) -> f32 {
        {
            use crate::Circle;
            2.0 * std::f32::consts::PI * circle.0
        }
    }
}
```

---

### It can be also used to reexport symbol

```rust
pub struct Circle(f32);

mod utils {
    mod inner {
        pub fn area(circle: &crate::Circle) -> f32 {
            2.0 * std::f32::consts::PI * circle.0
        }
    }

    pub use inner::area;
}
```

---

### It can be also used to reexport symbol

```rust
fn main() {
    let circle = Circle(4.0);
    println!("Area: {}", utils::area(&circle));
    // Compile error: `inner` module is not visible
    // println!("Area: {}", utils::inner::area(&circle));
}
```

---

### It can be also used to reexport symbol

```rust
pub struct Circle(f32);

mod utils {
    pub mod inner {
        pub(super) fn area(circle: &crate::Circle) -> f32 {
            2.0 * std::f32::consts::PI * circle.0
        }
    }

    pub use inner::area;
}
```

---

### It can be also used to reexport symbol

```rust
fn main() {
    let circle = Circle(4.0);
    println!("Area: {}", utils::area(&circle));
    // Compile error: `area` is visible only in `utils`
    // println!("Area: {}", utils::inner::area(&circle));
}
```

---

### Crate

Crate is a top-level module, typically in `main.rs` for binaries and `lib.rs` for libraries.

---

### Importing library crates

```toml
# Cargo.toml
[dependencies]
rand = "0.7"
```

```rust
// main.rs
fn main() {
    println!("Random number: {}", rand::threaded_rng().gen::<f32>());
}
```

---

### Importing library with additional options

```toml
# Cargo.toml
[dependencies]
old_tokio = { package = "tokio", version = "0.1", features = ["full"] }
```

---

### Useful options for library

* package - original crate name
* registry - registry to import crate from
* version - crate version (mandatory)
* features - features to enable in crate
* optional - dependency not imported by default, but can be enabled by feature

---

### Importing specific version of package

```toml
# Cargo.toml
[dependencies]
rand = "=0.7.1"
```

---

# Crates as modules

In most aspects crates behave as regular module. However importing crate, you gain access only to items declared with `pub` visibility, no matter if that is original definition, or reexport in one of `pub` submodules.


---

# Unit tests

```rust
#[test]
fn my_test() {}
```

```
$ cargo test
running 1 test
test my_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests ut

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

#### UT fails on panic

```rust
#[test]
fn my_test() {
    panic!("Failure");
}
```

```
$ running 1 test
test it_works ... FAILED
```

---

### There are usefull macros for comparison in tests

```rust
enum Shape {
    Circle(f32),
    Square(f32),
}

#[test]
fn my_test() {
    assert_eq!(10, 11); // Fails
    let circle = Circle(3.0);
    // Passes
    assert!(matches!(circle, Circle(radius) if radius == 3.0));
}
```

---

### Typically tests are enclosed in separate module

```rust
mod test {
    #[test]
    fn my_test() {
        // ...
    }
}
```

---

### Conditional compilation allows to compile tests only if they are needed

```rust
#[cfg(test)]
mod test {
    #[test]
    fn my_test() {
        // ...
    }
}
```

---

### It is possible to add dependency only for tests (and examples) build

```toml
# Cargo.toml
[dev-dependencies]
rand = "0.7"
```

```rust
fn main() {
    // Doesn't compile (unless running tests)
    use rand;
}

#[test]
fn my_test() {
    // Everything is fine
    use rand;
}
```

---

### Tests can be written as part of documentation

````rust
/// ```
/// panic!("Alert!")
/// ```
pub fn foo() {
}
````

```
$ running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests ut

running 1 test
test src/lib.rs - foo (line 1) ... FAILED
```

---

### Lines can be hidden in doctests with `#`

````rust
/// ```
/// # use rand::threaded_rng;
/// assert_eq!(threaded_rng().gen::<f32>(), 0.7f32);
/// ```
pub fn foo() {
}
````

---

### Any comment starting with `///` are considered documentation to following symbol

```rust
/// This function does very magical magic
pub fn foo() {
    abracadabra();
}
```

---

### Generating documentation

```
cargo doc --open
```

Doctests are usefull, because as every doc-comment they are visible in documentation, so they are usage example.

---

### Conditional compilation

* `#[cfg(test)]` - compile only if compiling test target
* `#[cfg(target_os="linux")]` - compile only on specific os
* `#[cfg(feature="feature_name")]` - compile only if feature is enabled
* `#[cfg(not(option))]`
* `#[cfg(any(option1, option2, ...))]`
* `#[cfg(all(option1, option2, ...))]`
* `#[cfg_attr(derive(Debug), test)]` - derive `Debug` for tests only

---

### Conditional compilation - features

Features to be used in conditional compilation has to be defined in `Cargo.toml`

```toml
[features]
default = ["features", "enabled", "by", "default"]
feature1 = []
# feature2 implies feature1
feature2 = ["feature1"]
# feature3 imports optional `rand` dependency
feature3 = ["rand"]
```

---

### Building with features enabled

```
cargo build --feature "feature1 feature3"
```
