# Ownership model

* Semantical move
* Borrows
    * Shared borrows
    * Mutable borrows
* Lifetimes
* Deriving

---

### By default objects are moved in Rust

```rust
struct Circle(f32);

fn area(Circle(radius): Circle) -> f32 {
    std::f32::consts::PI * radius * radius
}

fn main() {
    let circle = Circle(2.0);
    println!("Circle area: {}", area(circle));
    println!("Circle area: {}", area(circle));
}
```

---

### By default objects are moved in Rust

```
error[E0382]: use of moved value: `circle`
  --> src/main.rs:10:38
   |
8  |     let circle = Circle(2.0);
   |         ------ move occurs because `circle` has type `Circle`,
   |                which does not implement the `Copy` trait
9  |     println!("Circle area: {}", area(circle));
   |                                      ------ value moved here
10 |     println!("Circle area: {}", area(circle));
   |                                      ^^^^^^ value used here after move
```

Note:
In C++ we have move semantics, in Rust there is semantical move - moved object is nevermore available to use.

---

### Move in C++

```cpp
float area(Circle&& circle) {
    Circle local = std::move(circle);
    return std::numbers::pi_v<float> * local.radius * local.radius;
    // `local` destructor is called
}

int main() {
    auto circle = Circle(2.0);
    // `circle` moved to `area` - move constructor may be called, which leaves
    // old `circle` in after-move state
    std::cout << "Circle area: " << area(std::move(circle)) << '\n';
    // Circle may be a valid but unspecified object here - basically semantic UB.
    std::cout << "Circle area: " << area(std::move(circle)) << '\n';
    // `circle` destructor is called
}
```

---

### Move in Rust

```rust
fn area(Circle(radius): Circle) -> f32 {
    std::f32::consts::PI * radius * radius
    // `radius` is dropped here
}

fn main() {
    let circle = Circle(2.0);
    // `circle` moved to `area` - underlyind data are copied to new location,
    // and old variable is never usable (like never existed)
    println!("Circle area: {}", area(circle));
    // Compilation error - no such thing as circle here
    println!("Circle area: {}", area(circle));
    // Nothing is dropped here - circle is not existing anymore
}
```

---

### Moving into methods

```rust
impl Circle {
    fn area(self) -> f32 {
        std::f32::consts::PI * self.0 * self.0;
    }

    fn main() {
        let circle = Circle(2.0);
        println!("Circle area: {}", circle.area());
        // Same(ish) compilation error
        println!("Circle area: {}", circle.area());
    }
}
```

---

### Borrowing

```rust
fn area(Circle(radius): &Circle) -> f32 {
    std::f32::consts::PI * radius * radius
}

fn main() {
    let circle = Circle(2.0);
    println!("Circle area: {}", area(&circle));
    println!("Circle area: {}", area(&circle));
}
```

```
Circle area: 12.566371
Circle area: 12.566371
```

Note:
Borrow has to be taken explicitely
Borrows are very similar to references in C++, but they are comming with lifetimes

---

### Borrowing self

```rust
impl Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.0 * self.0
    }
}

fn main() {
    let circle = Circle(2.0);
    println!("Circle area: {}", circle.area());
    println!("Circle area: {}", circle.area());
}
```

```
Circle area: 12.566371
Circle area: 12.566371
```

Note:
For method object doesn't need to borrow explicitely

---

### Mutating through borrow

```rust
fn main() {
    let mut number = 3.0;
    let borrow: &f32 = &number;
    *borrow = 2.0;
}
```

```
error[E0594]: cannot assign to `*borrow` which is behind a `&` reference
 --> src/main.rs:4:5
  |
3 |     let borrow = &number;
  |                  ------- help: consider changing this to be a mutable
  |                                reference: `&mut number`
4 |     *borrow = 2.0;
  |     ^^^^^^^^^^^^^ `borrow` is a `&` reference, so the data it refers
  |                   to cannot be written
```

---

### Mutating throug borrow

```rust
fn main() {
    let mut number = 3.0;
    let borrow: &mut f32 = &mut number;
    *borrow = 2.0;
    println!("Number: {}", number);
}
```

```
Number: 2
```

---

### Borrowing rules

Variable can be borrowed:
* Only once via mutable borrow, OR
* Any number of time via shared borrow

Note:
Shared borrow is not *immutable* borrow

---

### Mutable borrow when shared borrow exists

```rust
fn main() {
    let mut number = 2.0;
    let borrow = &number;
    // Compile error - variable borrowed before
    let mut_borrow = &mut borrow;
    println!("{}", borrow);
}
```

---

### Shared borrow when mutable borrow exists

```rust
fn main() {
    let mut number = 2.0;
    let mut_borrow = &mut number;
    // Compile error - variable borrowed mutably before
    let borrow = &borrow;
    println!("{}", mut_borrow);
}
```

---

### Mutable borrow when mutable borrow exists

```rust
fn main() {
    let mut number = 2.0;
    let mut_borrow = &mut number;
    // Compile error - variable borrowed mutably before
    let mut_borrow2 = &number;
    println!("{}", mut_borrow);
}
```

---

### Borrow is considered alive only if is later used

```rust
fn main() {
    let mut number = 2.0;
    let borrow = &number;
    println!("Number: {}", borrow);
    let mut_borrow = &mut number;
    *mut_borrow = 3.0;
    println!("Number: {}", number);
}
```

```
Number: 2
Number: 3
```

---

### While variable is borrowed it cannot be mutated at all

```rust
fn main() {
    let mut number = 2.0;
    let borrow = &number;
    // Compilation error - number is borrowd
    number = 3.0;
    println!("Number: {}", borrow);
}
```

---

### Borrow cannot outlife object it borrows from

```rust
fn main() {
    let borrow = {
        let number = 2.0;
        let borrow = &number;
        borrow
    };
    println!("Number: {}", borrow);
}
```

---

### Borrow cannot outlife object it borrows from

```
error[E0597]: `number` does not live long enough
 --> src/main.rs:4:22
  |
2 |     let borrow = {
  |         ------ borrow later stored here
3 |         let number = 2.0;
4 |         let borrow = &number;
  |                      ^^^^^^^ borrowed value does not live long enough
5 |         borrow
6 |     };
  |     - `number` dropped here while still borrowed
```

---

### Borrow to local variable cannot be returned

```rust
fn foo() -> &f32 {
    let number = 2.0;
    &number
}
```

```
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:13
  |
1 | fn foo() -> &f32 {
  |             ^ help: consider giving it a 'static lifetime: `&'static`
  |
  = help: this function's return type contains a borrowed value, but there
  |       is no value for it to be borrowed from
```

---

### Try to fix it with compiler hint...

```rust
fn foo() -> &'static f32 {
    let number = 2.0;
    &number
}
```

```
error[E0515]: cannot return reference to local variable `number`
 --> src/lib.rs:3:5
  |
3 |     &number
  |     ^^^^^^^ returns a reference to data owned by the current function
```

---

### Borrows can be returned, if they were taken in argument

```rust
fn foo(borrow: &f32) -> &f32 {
    borrow
}
```

---

### But how compiler knows which borrow is returned?

```rust
fn foo(left: &f32, right: &f32) -> &f32 {
    right
}
```

```
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:36
  |
1 | fn foo(left: &f32, right: &f32) -> &f32 {
  |                                    ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
          signature does not say whether it is borrowed from `left` or `right`
help: consider introducing a named lifetime parameter
  |
1 | fn foo<'lifetime>(left: &f32, right: &f32) -> &'lifetime f32 {
  |       ^^^^^^^^^^^                             ^^^^^^^^^^
```

---

### This is where lifetimes comes into play

```rust
fn foo<'a>(left: &f32, right: &'a f32) -> &'a f32 {
    right
}

fn main() {
    let right = 2.0;
    let res = {
        let left = 3.0;
        foo(&left, &right)
    };
    println!("Result: {}", res);
}
```

```
Result: 2
```

---

### What actually lifetime means?

```rust
fn foo<'a>(
//     ^^ There is some point in time...
    left: &f32,
    right: &'a f32
//          ^^ ...which is (1) guaranteed not to be outliven by this borrow...
) -> &'a f32 {
//   ^^ ...and (2) cannot be outliven by this borrow.
    right
}
```

It is compiler job to prove, that assuming (1), (2) is true.

---

### Returning field of borrowed struct

```rust
fn radius<'a>(Circle(radius): &'a Circle) -> &'a f32 {
    radius
}

fn main() {
    let circle = Circle(2.0);
    println!("Circle radius: {}", radius(&circle));
}
```

```
Circle radius: 2
```

---

### Lifetime can be omitted, when there is only one borrowed argument

```rust
fn radius(Circle(radius): &Circle) -> &f32 {
    radius
}

fn main() {
    let circle = Circle(2.0);
    println!("Circle radius: {}", radius(&circle));
}
```

```
Circle radius: 2
```

---

### When `self` is taken as borrow, lifetime always can be ommited - they would be elided to `self` lifetime

```rust
impl Circle {
    fn radius(&self, context: &f32) -> &f32 {
        &self.0
    }
}
```

---

### When `self` is taken as borrow, lifetime always can be ommited - they would be elided to `self` lifetime

```rust
impl Circle {
    fn radius(&self, context: &f32) -> &f32 {
        // Compiler error
        context
    }
}
```

---

### When `self` is taken as borrow, lifetime always can be ommited - they would be elided to `self` lifetime

```
error[E0623]: lifetime mismatch
 --> src/lib.rs:5:9
  |
4 |     fn radius(&self, context: &f32) -> &f32 {
  |                               ----     ----
  |                               |
  |                               this parameter and the return type are
  |                               declared with different lifetimes...
5 |         context
  |         ^^^^^^^ ...but data from `context` is returned here
```

---

### When returing borrow, the whole input borrow is kept, so it cannot be mutated

```rust
fn edge1(Rectangle(edge1, _): &Rectangle) -> &f32 {
    edge1
}

fn main() {
    let mut rect = Rectangle(3.0, 4.0);
    let borrow = edge1(&rect);
    rect.1 = 5.0;
    println!("First edge: {}", borrow);
}
```

---

### When returing borrow, the whole input borrow is kept, so it cannot be mutated

```
   Compiling playground v0.0.1 (/playground)
error[E0506]: cannot assign to `rect.1` because it is borrowed
  --> src/main.rs:10:5
   |
9  |     let borrow = edge1(&rect);
   |                        ----- borrow of `rect.1` occurs here
10 |     rect.1 = 5.0;
   |     ^^^^^^^^^^^^ assignment to borrowed `rect.1` occurs here
11 |     println!("First edge: {}", borrow);
   |                                ------ borrow later used here
```

---

### Mutable borrows can be used to mutate function argument

```rust
fn enlarge(Circle(radius): &mut Circle, scalar: f32) {
    *radius *= scalar;
}

fn main() {
    let mut circle = Circle(2.0);
    enlarge(&mut circle, 1.5);
    println!("New radius: {}", circle.0);
}
```

```
New radius: 3
```

---

### This can be done to mutate via `self` too

```rust
impl Circle {
    fn enlarge(&mut self, scalar: f32) {
        self.0 *= scalar;
    }
}

fn main() {
    let mut circle = Circle(2.0);
    circle.enlarge(2.0);
    println!("New radius: {}", circle.0);
}
```

```
New radius: 4
```

---

### But keep in mind, that borrowing mutably keeps borrow until returned borrow is alive...

```rust
fn enlarge(Circle(radius): &mut Circle, scalar: f32) -> &f32 {
    *radius *= scalar;
    radius
}

fn main() {
    let mut circle = Circle(2.0);
    let radius = enlarge(&mut circle, 1.5);
    let borrow = &circle;
    println!("New radius: {}", radius);
}
```

---

### But keep in mind, that borrowing mutably keeps borrow until returned borrow is alive...

```
error[E0502]: cannot borrow `circle` as immutable because it is also
              borrowed as mutable
  --> src/main.rs:11:18
   |
10 |     let radius = enlarge(&mut circle, 1.5);
   |                          ----------- mutable borrow occurs here
11 |     let borrow = &circle;
   |                  ^^^^^^^ immutable borrow occurs here
12 |     println!("New radius: {}", radius);
   |                                ------ mutable borrow later used here
```

---

### `'static` borrow is a borrow valid through whole application - basicly borrow to compile time constant

```rust
const HELP: &str = "HELP";

fn help() -> &'static str {
    HELP
}

fn hello() -> &'static str {
    "hello"
}

fn point() -> &'static Circle {
    &Circle(0.0)
}
```

---

### Lifetimes are used to keep references inside types

```rust
struct Circle<'r> {
//            ^^ for given lifetime `'r`
    radius: &'r f32,
//           ^^ `radius` cannot outlife `'r`
}

fn main() {
    let circle = {
        let radius = 3.0;
        Circle {
            radius: &radius,
        }
    };
}
```

---

### Lifetimes are used to keep references inside types

```
error[E0597]: `radius` does not live long enough
  --> src/main.rs:11:21
   |
8  |     let circle = {
   |         ------ borrow later stored here
...
11 |             radius: &radius,
   |                     ^^^^^^^ borrowed value does not live long enough
12 |         }
13 |     };
   |     - `radius` dropped here while still borrowed
```

---

### Such struct can be obviously returned from function

```rust
fn make_circle<'r>(radius: &'r f32) -> Circle<'r> {
    Circle { radius }
}

fn main() {
    let circle = make_circle(&4.0);
    println!("Radius: {}", circle.radius);
}
```

```
Radius: 4
```

---

### Lifetime generic can be omitted

```rust
fn make_circle(radius: &f32) -> Circle {
    Circle { radius }
}

fn main() {
    let circle = make_circle(&4.0);
    println!("Radius: {}", circle.radius);
}
```

```
Radius: 4
```

---

### Generic lifetime can be also used with traits

```rust
trait HasRadius<'r> {
    fn radius(&self) -> &'r f32;
}

impl<'r> HasRadius<'r> for Circle<'r> {
    fn radius(&self) -> &'r f32 {
        self.radius
    }
}
```

Note that returned borrow is related to lifetime bounded to structure, not to `self`

---

### Generic lifetime can be also used with traits

```rust
fn main() {
    let radius = 3.0;
    let borrow = {
        let circle = Circle { radius };
        circle.radius()
    };
    println!("Radius: ", borrow);
}
```
```
Radius: 3.0
```

---

### `Copy` trait

`Copy` trait instructs compiler, that objects of the type is cloned instead of moved. Always when `Copy` type needs to be moved, `Clone::clone(&self)` function is called on it (so it also needs to implement `Clone` trait.

```rust
fn add(l: f32, r: f32) -> f32 {
    l + r
}

fn main() {
    let number = 3.0; // primitive types implements `Copy` trait
    // Nothing is moved here
    println!("number + 2 = {}", add(number, 2.0));
    // Number still can be used later
    println!("number + 4 = {}", add(number, 4.0));
}
```

---

### Deriving `Copy` and `Clone`

```rust
#[derive(Clone, Copy)]
struct Circle(f32);

fn area(Circle(radius): Circle) -> f32 {
    std::consts::f32::PI * radius * radius
}

fn main() {
    let circle = Circle(2.0);
    println!("Area: {}", area(circle));
    println!("Area: {}", area(circle));
}
```
```
Area: 12.566371
Area: 12.566371
```

---

### Cloning non-copy structs

```rust
#[derive(Clone)]
struct Circle(f32);

fn area(Circle(radius): Circle) -> f32 {
    std::consts::f32::PI * radius * radius
}

fn main() {
    let circle = Circle(2.0);
    println!("Area: {}", area(circle.clone()));
    println!("Area: {}", area(circle.clone()));
}
```
```
Area: 12.566371
Area: 12.566371
```

---

### Deriving other usefull traits

```rust
#[derive(Debug)]
struct Circle(f32);

fn main() {
    let circle = Circle(2.0);
    println!("Circle: {:?}", circle);
}
```
```
Circle: Circle(2.0)
```

---

### Deriving other usefull traits

* `Clone`, `Copy`
* `Debug`
* `Eq`, `PartialEq`, `Ord`, `PartialOrd`
* `Default`
* `Hash`
