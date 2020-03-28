## Module system and typesystem basics

* Using standard library and documentation
* `struct`
* `enum` as a sum type
* basic `match` statement
* `trait` as an interface
* tuples, tuple `structs`
* arrays

---

### Standard library

Rust programs are organized in modules. Rust standard library is delivered in module named `std`. Sometimes standard library might be disabled (for example for embeded devices without heap), in such cases there is also a `core` module which delivers core library functionality which can be used always, no matter regardless environment capabilities.

Entities inside modules are addressed like in C++ namespaces - with module name and `::`, for example `std::fs::File` is a type to handling files in Rust.

---

### `use` keyword

`use` can be used to bring entity from inside the module to the scope of current module, for example:

```rust
use std::fs::File;

let file = File::open("file.txt").unwrap();
```

```rust
use std::fs;

let file = fs::File::open("file.txt").unwrap();
```

---

### `use` grouping

Use statements can be grouped inside curly braces, and `*` can be used to import whole module. Special keyword `self` can be used to import module itself in grouped `use`:

```rust
// Imports `File` and `FileType` form `std::fs` module,
// as well as `fs` module from `std` itself
use std::fs::{File, FileType, self};

// Imports everything from `std::collections` module
use std::collections::*;

```

---

### Standard library documentation and prelude

<https://doc.rust-lang.org/std/index.html>

By default everything from `std::prelude` is imported into global scope: <https://doc.rust-lang.org/std/prelude/index.html>.

In case of compiling wihtout standard library, the `core::prelude::v1` is imported instead: <https://doc.rust-lang.org/core/prelude/v1/index.html>.

---

### `struct` type

Structures are rust product types - equivalents of C++ `struct` and `class`.

```rust
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let point = Point {
        x: 5.0,
        y: 3.0,
    };

    println!("x: {}, y: {}", point.x, point.y);
}
```

---

### `enum` as a sum type

Enums can be used to express sum types - equivalents of C++ `enum class`.

```rust
enum Shape {
    Circle,
    Square,
    Rombus,
    Rectangle,
}

fn main() {
    let shape = Shape::Rombus;
    if shape == Shape::Circle { println!("Circle"); }
    else if shape == Shape::Square { println!("Square"); }
    else if shape == Shape::Rombus { println!("Rombus"); }
    else { println!("Rectangle"); }
}
```

---

### Comparing enums

```
error[E0369]: binary operation `==` cannot be applied to type `Shape`
  --> src/main.rs:10:14
   |
10 |     if shape == Shape::Circle { println!("Circle"); }
   |        ----- ^^ ------------- Shape
   |        |
   |        Shape
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for
           `Shape`

...
```

But they cannot be compared by default! :( Printing would also not work. We will learn how to deal with it later, for now we would find some tools to use something usefull with `enums`.

---

###  Basic `match` statement

```rust
match expression {
    value1 => expr1,
    value2 => { expr2 }
    value3 => expr3,
}
```

Expressions in match statemets can be either single expressions or blocks.

---

### Matching enums

```rust
fn main() {
    let shape = Shape::Rombus;
    match shape {
        Shape::Circle => println!("Circle"),
        Shape::Square => println!("Square"),
        Shape::Rombus => println!("Rombus"),
        Shape::Rectangle => println!("Rectangle"),
    }
}
```

```
Rombus
```

---

### Matches has to be exhaustive

```rust
match shape {
    Shape::Circle => println!("Circle"),
    Shape::Square => println!("Square"),
    Shape::Rombus => println!("Rombus"),
}
```

```
error[E0004]: non-exhaustive patterns: `Rectangle` not covered
  --> src/main.rs:10:7
   |
1  | / enum Shape {
2  | |     Circle,
3  | |     Square,
4  | |     Rombus,
5  | |     Rectangle,
   | |     --------- not covered
6  | | }
   | |_- `Shape` defined here
...
10 |   match shape {
   |         ^^^^^ pattern `Rectangle` not covered
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
```

---

### `_` pattern

```rust
let n = 5;

match n {
    0 => println!("0"),
    1 => println!("1"),
    _ => println!("More than 1, or negative"),
}
```

```
More than 1, or negative
```

---

### Just like `if`, `match` can be used as expression - all branches has to return the same type

```rust
fn main() {
    let shape = Shape::Rombus;
    let corners = match shape {
        Shape::Circle => 0,
        _ => 4,
    };

    println!("Shape has {} corners", corners);
}
```

```
Shape has 4 corners
```

---

### Structs can have methods defined on them

```rust
impl Point {
    fn dist(left: Point, right: Point) -> f32 {
        let xdiff = left.x - right.x;
        let ydiff = left.y - right.y;
        (xdiff * xdiff + ydiff * ydiff).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 6.0 };
    println!("distance: {}", Point::dist(p1, p2));
}
```

```
distance: 2.828427
```

Methods are "static" by default.

---

### `self` keyword as `this` equivalent

```rust
impl Point {
    fn dist(self, right: Point) -> f32 {
        let xdiff = self.x - right.x;
        let ydiff = self.y - left.y;
        (xdiff * xdiff + ydiff * ydiff).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 6.0 };
    println!("distance: {}", p1.dist(p2));
}
```

```
distance: 2.828427
```

`self` is always the first method argument, and is in general defined without type.

---

### Methods can be also defined on enums

```rust
impl Shape {
    fn corners(self) -> u8 {
        match self {
            Self::Circle => 0,
            _ => 4,
        }
    }
}

fn main() {
    let shape = Shape::Circle;
    println!("Shape has {} corners", shape.corners());
}
```

```
Shape has 0 corners
```

`Self` is an alias for type on which method is defined.

---

### Convention: `new` function

```rust
impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point {
            x,
            y,
        }
    }
}
```

There are no speciall constructors in rust - there is a convention to use `new` function for default constructor. Note, that `field: field` can be simplified to `field` on function construction.

---

### Traits as interfaces

```rust
trait Shape {
    fn area(self) -> f32;
}

struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}
```

---

### Traits as interfaces

```rust
struct Square {
    edge: f32,
}

impl Shape for Square {
    fn area(self) -> f32 {
        self.edge * self.edge
    }
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let square = Square { edge: 3.0 };
    println!("square area: {}, circle area: {}", square.area(), circle.area());
}
```

```
square area: 9, circle area: 28.274334
```

---

### Tuples

```rust
let x = (1, 3.23, Circle { radius: 2.0 });

println!("{} {} {}", x.0, x.1, x.2.area());
```

```
1 3.23 12.566371
```

Tuples are building types in rust.

Unit type `()` is a special 0-element struct.

---

### One element tuple

```rust
let x = (1);
println!("{}", x.0);
```

```
error[E0610]: `{integer}` is a primitive type and therefore doesn't have
              fields
  --> src/main.rs:27:18
   |
27 | println!("{}", x.0);
   |                  ^
```

What was intended to be one element tuple evaluates to simple expression in brackets!

---

### One element tuple

```rust
let x = (1,);
println!("{}", x.0);
```

```
1
```

One element tuples shoud contain additional comma to distinguish them from expressions in brackets.

---

### Tuple structs

```rust
struct Point(f32, f32);

impl Point {
    fn dist(self, right: Point) -> f32 {
        let xdiff = self.0 - right.0;
        let ydiff = self.1 - right.1;
        (xdiff * xdiff + ydiff * ydiff).sqrt()
    }
}

fn main() {
    let p1 = Point(2.0, 4.0);
    let p2 = Point(7.0, 11.0);
    println!("distance: {}", p1.dist(p2));
}
```

```
distance: 8.602325
```

Note:
Tuple structs are exact equivalents of struct, except their fields are addressed as tuple fields

---

### Arrays

```rust
let arr: [u32; 4] = [1, 2, 3, 4];

println!(
    "[0]: {}, [1]: {}, [2]: {}, [3]: {}",
    arr[0], arr[1], arr[2], arr[3],
);
```

```
[0]: 1, [1]: 2, [2]: 3, [3]: 4
```

Array type could be obviously elided - it is just to demonstrate array type syntax.

---

### Tuples can be destucturised

```rust
let tpl = (1, 3.14);
let (one, pi) = tpl;
println!("one: {}, pi: {}", one, pi);
```

```
one: 1, pi: 3.14
```

---

### And also structs can

```rust
let point = Point(3.0, 4.0);
let circle = Circle { radius: 2.0 };

let Point(x, y) = point;
let Circle { radius } = circle;

println!("x: {}, y: {}, radius: {}", x, y, radius);
```

```
x: 3, y: 4, radius: 2
```

---

### And arrays too

```rust
let arr = [1, 2];
let [one, two] = arr;

println!("one: {}, two: {}", one, two);
```

```
one: 1, two: 2
```

---

### Fields ommiting and renaming

```rust
struct Rectangle {
    a: f32,
    b: f32,
}

fn main() {
    let rect = Rectangle { a: 5.0, b: 6.0 };
    let Rectangle {
        a: important_edge,
        ..
    } = rect;

    println!("important_edge: {}", important_edge);
}
```

```
important_edge: 5
```

---

### Practice

Create tic-tac-toe game. Players should be defined as an enum, and the game should have functions:
* `player` which gives back player to perform move
* `move` taking field number, where current player makes move; function should return tuple of new state, and game outcome

`Outcome` should be an enum with values `Ongoing`, `XWon`, `OWon`, `Draw`
