# Generics

* Generic functions
* Trait bounds
* Generic types
* Generic traits
* Associated types and constants
* Lifetimes as generics
* Dependent traits
* Extension traits

---

### Generics allow to pass an object without knowledge about its type

```rust
fn length<T>(v: &Vec<T>) -> usize {
    v.len()
}

fn main() {
    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<String> = vec!["A".to_owned(), "Foo".to_owned()];

    println!("{}, {}", length(&v1), length(&v2));
}
```

```
3, 2
```

---

### However in contrast to C++, in Rust generics doesn't allow to do any funny thing with generics

```rust
fn add<T>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
}
```

---

### However in contrast to C++, in Rust generics doesn't allow to do any funny thing with generics

```
   Compiling playground v0.0.1 (/playground)
error[E0369]: cannot add `T` to `T`
 --> src/main.rs:2:7
  |
2 |     a + b
  |     - ^ - T
  |     |
  |     T
  |
  = note: `T` might need a bound for `std::ops::Add`
  ```

---

### Information what can be done with a type has to be known basing on signature

```rust
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>
{
    a + b
}

fn main() {
    println!("Result: {}", add(2, 3));
}
```
```
Result: 5
```

---

### But this can be even more generic

```rust
fn add<T>(a: T, b: T) -> T::Output
where
    T: std::ops::Add
{
    a + b
}

fn main() {
    println!("Result: {}", add(2, 3));
}
```
```
Result: 5
```

---

### `where` clause can be ommited, and bounds can be defined on generics declaration

```rust
fn add<T: std::ops::Add>(a: T, b: T) -> T::Output {
    a + b
}

fn main() {
    println!("Result: {}", add(2, 3));
}
```
```
Result: 5
```

---

### It is possible to have bounds depending on other generic types

```rust
fn add_and_print<T, R>(a: T, b: T)
where
    T: std::ops::Add<Output = R>,
    R: std::fmt::Display,
{
    println!("Result: {}", a + b);
}

fn main() {
    add_and_print(1, 4);
}
```
```
Result: 5
```

---

### Return value can also be a generic type

```rust
fn collect_squares<Output>(count: u32) -> Output
where
    Output: std::iter::FromIterator<u32>,
{
    fn square(i: u32) -> u32 {
        i * i
    }

    (1..=count).map(square).collect()
}

fn main() {
    println!("First {} squares: {:?}", 10, collect_squares::<Vec<u32>>(10));
}
```
```
First 10 squares: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
```

---

### Unlike in C++, only part of type which cannot be deduced, have to be provided

```rust
fn collect_squares<Output>(count: u32) -> Output
where
    Output: std::iter::FromIterator<u32>,
{
    (1..=count).map(|i| i * i).collect()
}

fn main() {
    println!("First {} squares: {:?}", 10, collect_squares::<Vec<_>>(10));
}
```
```
First 10 squares: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
```

---

### Actually whole return type can be deduced if context allows it

```rust
fn collect_squares<Output>(count: u32) -> Output
where
    Output: std::iter::FromIterator<u32>,
{
    (1..=count).map(|i| i * i).collect()
}

fn main() {
    let count = 10;
    let squares: Vec<_> = collect_squares(count);
    println!("First {} squares: {:?}", count, squares);
}
```
```
First 10 squares: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
```

---

### Actually whole return type can be deduced if context allows it

```rust
fn collect_squares<O: std::iter::FromIterator<u32>>(count: u32) -> O
{
    (1..=count).map(|i| i * i).collect()
}

fn print_vec<T: std::fmt::Debug>(v: Vec<T>) {
    println!("{:?}", v);
}

fn main() {
    let squares = collect_squares(10);
    print_vec(squares);
}
```
```
[1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
```

---

### Multiple bounds can be set on single genric

```rust
fn lin_transform<T>(a: T, b: T, c: T) -> T
where
    T: std::ops::Add<Output = T>,
    T: std::ops::Mul<Output = T>,
{
    a * b + c
}

fn main() {
    println!("Result: {}", lin_transform(3.2, 5.7, 4.5));
}
```
```
Result: 22.740000000000002
```

---

### Multiple bounds can be compounded with `+`

```rust
fn lin_transform<T>(a: T, b: T, c: T) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    a * b + c
}

fn main() {
    println!("Result: {}", lin_transform(3.2, 5.7, 4.5));
}
```
```
Result: 22.740000000000002
```

---

### `impl Trait` on argument position

```rust
fn print_iterator(iter: impl Iterator<Item = u32>) {
    let v: Vec<_> = iter.collect();
    println!("{:?}", v);
}

fn main() {
    print_iterator(4..9);
}
```
```
[4, 5, 6, 7, 8]
```

---

### `impl Trait` on arguments associated types

```rust
fn print_iterator(iter: impl Iterator<Item = impl std::fmt::Debug>) {
    let v: Vec<_> = iter.collect();
    println!("{:?}", v);
}

fn main() {
    print_iterator(4..9);
}
```
```
[4, 5, 6, 7, 8]
```

---

### `impl Trait` on return position

```rust
fn squares(count: u32) -> impl Iterator<Item = u32> {
    (1..=count).map(|i| i * i)
}

fn main() {
    let count = 10;
    let squares: Vec<_> = squares(count).collect();
    println!("Squares: {:?}", squares);
}
```
```
Squares: [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]
```

---

### Generics in struct

```rust
struct Circle<T> {
    radius: T,
}

impl<T> Circle<T>
where
    T: std::ops::Mul<f64> + Copy,
{
    fn circumference(&self) -> T::Output {
        self.radius * (std::f64::consts::PI * 2.0)
    }
}
```

---

### Generics in struct

```rust
fn main() {
    let circle = Circle { radius: 2.0 };
    println!("Circumference: {}", circle.circumference());
    let circle = Circle { radius: () };
    println!("Circumference: {}", circle.circumference())
}
```
```
   Compiling playground v0.0.1 (/playground)
error[E0599]: no method named `circumference` found for struct `Circle<()>`
              in the current scope
  --> src/main.rs:18:42
   |
1  | struct Circle<T> {
   | ---------------- method `circumference` not found for this
...
18 |     println!("Circumference: {}", circle.circumference())
   |                                          ^^^^^^^^^^^^^ method not found
   |                                                        in `Circle<()>`
   |
   = note: the method `circumference` exists but the following trait bounds
           were not satisfied: `() : std::ops::Mul<f64>`
```

---

### Generics in struct

```rust
fn main() {
    let circle = Circle { radius: 2.0 };
    println!("Circumference: {}", circle.circumference());
}
```
```
Circumference: 12.566370614359172
```

---

### Generics in struct
```rust
impl<T> Circle<T>
where
    T: std::ops::Mul<f64> + Copy,
    T: std::ops::Mul<Output = T>,
{
    fn circumference(&self) -> T::Output {
        self.radius * (std::f64::consts::PI * 2.0)
    }

    fn area(&self) -> T::Output {
        self.radius * self.radius * std::f32::consts::PI
    }
}
```

---

### Struct generics can be bounded for specific functions

```rust
impl<T> Circle<T>
where
    T: std::ops::Mul<f64> + Copy,
{
    fn circumference(&self) -> T::Output {
        self.radius * (std::f64::consts::PI * 2.0)
    }

    fn area(&self) -> T::Output
    where T: std::ops::Mul<Output = T>,
    {
        self.radius * self.radius * std::f32::consts::PI
    }
}
```

---

### Struct generics can be used in mixture with method generics

```rust
impl<T> Circle<T>
{
    fn scale<S, R>(scale: S) -> R
    where
        T: std::ops::Mul<S, Output = R> + Copy
    {
        self.radius * scale
    }
}
```

---

### Generics on types can have default values

```rust
struct Circle<T=f32> {
    radius: T,
}
```

---

### Generic traits

```rust
struct Circle(f32);

trait Shape<T> {
    fn circumference(&self) -> T;
    fn area(&self) -> T;
}

impl Shape<f32> for Circle {
    // ...
}

impl Shape<f64> for Circle {
    // ...
}
```

---

### Generic traits and types can interop

```rust
struct Circle<T>(T);

impl<T> Shape<T> for Circle<T>
where
    T: Mul<f64, Output = T>,
    T: Mul<Output = T>
{
    // ...
}
```

---

### Traits associated types

```rust
struct Circle(f32);

trait Shape {
    type Output;

    fn area(&self) -> Self::Output;
}

impl Shape for Circle {
    type Output = f32;

    fn area(&self) -> f32 {
        // ...
    }
}
```

---

### Associated types interworking with generics

```rust
struct Circle<T>(T);

impl<T> Shape for Circle<T> 
where
    T: Mul<f64, Output = T>,
    T: Mul<Output = T>,
{
    type Output = T;

    fn area(&self) -> T {
        // ...
    }
}
```

---

### Associated types can be also bound to traits

```rust
trait Shape {
    type Output: std::fmt::Display;

    fn area(&self) -> Self::Output;
    fn print_area(&self) {
        println!("Area: {}", self.area());
    }
}
```

---

### Associated types vs generic traits

* Traits with associated types can only be implemented once, and the type is determined by implementation
* Traits generic over type can be implemented multiple times, always with different generic

---

### Associated types vs generic traits

#### `Iterator`

Associated `Output` type - if something is an iterator, it yields constantly one type of objects

---

### Associated types vs generic traits

#### `From<T>`/`Into<T>`

Type can be converted from/to diffrent types, for every conversion one of those traits are implemented.

---

### Associated constants

```rust
trait Shape {
    const VERTICES: u8;
}

impl Shape for Circle {
    const VERTICES: u8 = 0;
}

impl Shape for Triangle {
    const VERTICES: u8 = 3;
}
```

---

### Associated constants

```rust
fn main() {
    let circle = Circle(3.0);
    let traingle = Triangle(2.0, 5.0, 9.0);
    println!(
        "Circle has {} corners, traingle has {} of them",
        Circle::VERTICES,
        Triangle::VERTICES,
    );
}
```
```
Circle has 0 corners, traingle has 3 of them
```
---

### Lifetime as a trait

```rust
fn magic_function<T: 'static>(arg: T) {
//                ^^^^^^^^^^
//                This type cannot be bounded by any lifetime but static
// ...
}
```

---

# But what does it even mean?

---

### Try to call the function with some random type

```rust
fn main() {
    let circle = Circle(2.0);
    magic_function(circle);
}
```

---

### Ok, now try to do some borrow

```rust
fn main() {
    let data: &str = "Some string";
    magic_function(data);
}
```

---

### So everything works? Why is this bound even needed?

```rust
fn main() {
    let data = 5;
    let borrowed = &data;
    magic_function(borrowed);
}
```
```
error[E0597]: `data` does not live long enough
  --> src/main.rs:10:20
   |
10 |     let borrowed = &data;
   |                    ^^^^^ borrowed value does not live long enough
11 |     magic_function(borrowed);
   |     ------------------------ argument requires that `data` is borrowed
   |                              for `'static`
12 | }
   | - `data` dropped here while still borrowed
```

---

### Also types generic over lifetimes would be a problem

```rust
struct Circle<'r>(&'r f32);

fn main() {
    let radius = 4.0;
    let circle = Circle(&radius);
    magic_function(circle);
}
```
```
error[E0597]: `radius` does not live long enough
  --> src/main.rs:10:25
   |
10 |     let circle = Circle(&radius);
   |                         ^^^^^^^ borrowed value does not live long enough
11 |     magic_function(circle);
   |     ---------------------- argument requires that `radius` is borrowed
   |                            for `'static`
12 | }
   | - `radius` dropped here while still borrowed
```

---

### `'static` lifetime means, that object is valid for entire application

* Every trivial type is `'static`
* Every borrow to compile time constant is `'static`
* Every compound type containing only `'static` fields is `'static`

---

### But can other lifetimes be used as a trait bound?

```rust
fn magic_function<'a, T: 'a>(_: &'a u32, _: T) -> &'a {
}

fn main() {
    let number = 4;
    magic_function(&number, 10);
}
```

---

### All static types would work - `'static` is "bigger" lifetime than any other

```rust
struct Circle(f32);

fn main() {
    let number = 4;
    let circle = Circle(2.0);
    magic_function(&number, circle);
}
```

---

### It works also for variables having common lifetime

```rust
fn main() {
    let number = 4;
    let circle = Circle(2.0);
    magic_function(&number, &circle);
}
```

---

### Bot wont, if lifetimes are unrelated

```rust
fn callee<'a>(number: &'a u32, circle: &Circle) -> 'a &u32 {
    magic_function(number, circle)
}
```
```
error[E0623]: lifetime mismatch
 --> src/main.rs:8:5
  |
7 | fn calee<'a, 'b>(number: &'a u32, circle: &'b Circle) -> &'a u32 {
  |                                           ----------     -------
  |                                           |
  |                                           this parameter and the return type are declared with different lifetimes...
8 |     magic_function(number, circle)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...but data from `circle` is returned here
```

---

### Same for structures with lifetimes inside

```rust
struct Circle<'r>(&'r f32);

fn callee<'a, 'b>(number: &'a u32, circle: Circle<'b>) -> &'a u32 {
    magic_function(number, circle)
}
```

---

### Lifetimes relations

```rust
fn callee<'a, 'b: 'a>(number: &'a u32, &'b u32) -> &'a u32 {
    magic_function(number, circle)
}
```

---

### `static` lifetime again

`'static` is lifetime, which for any given lifetime `'a` fullfils:

`'static: 'a`

---

### Dependent traits

```rust
trait Shape: std::fmt::Debug {
    fn area(&self) -> f64;
    fn print(&self) {
        println!("Shape {:?} has area of {}", self, self.area());
    }
}

struct Circle(f64);

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
}
```

---

### Dependent traits

```
   Compiling playground v0.0.1 (/playground)
error[E0277]: `Circle` doesn't implement `std::fmt::Debug`
  --> src/lib.rs:10:6
   |
10 | impl Shape for Circle {
   |      ^^^^^ `Circle` cannot be formatted using `{:?}`
   |
   = help: the trait `std::fmt::Debug` is not implemented for `Circle`
   = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
```

---

### Dependent traits

```rust
#[derive(Debug)]
struct Circle(f64);

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
}

fn main() {
    let circle = Circle(4.0);
    circle.print();
}
```

```
Shape Circle(4.0) has area of 50.26548245743669
```

---

### Extension traits

```rust
trait AvgIterator: Iterator + Sized {
    fn avg(self) -> f64
    where
        Self::Item: Default + AddAssign + Mul<f64, Output = f64>,
    { 
        let mut sum = Self::Item::default();
        let mut cnt = 0;
        for item in self {
            sum += item;
            sum = sum + 1;
        }
        sum * (1.0 / cnt as f64)
    }
}
```

---

### Extension traits

```rust
impl<I: Iterator> AvgIterator for I {}

fn main() {
    let avg = (1..4).map(|i| i as f64).avg();
    println!("Average: {}", avg);
}
```
```
Average: 2
```
