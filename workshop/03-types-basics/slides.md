## Rust typesystem

* `struct`
* `enum`
* `match` statement
* `if let` statement
* `while let` statement
* `trait` as an interface
* tuples, tuple `structs`
* arrays

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

### `enum` as a algebraic type

Enums can be also used as combination of sum and product types - C++ equivalent of "tagged union" - combination of `union` compounded with `enum class` performing as actual tag.

```rust
enum Shape {
    Circle(f32),
    Square(f32),
    Rombus(f32, f32),
    Rectangle(f32, f32),
}
```

This means, that `Shape` contains additional data depending on the variant it represents. Lets try to do something usefull with it with match:

---

### Destructurization in `match`

```rust
fn main() {
    let shape = Shape::Rectangle(10.0, 12.0);
    match shape {
        Shape::Circle(radius) =>
            println!("Circle of radius {}", radius),
        Shape::Square(edge) =>
            println!("Square of edge {}", edge),
        Shape::Rombus(d1, d2) =>
            println!("Rombus of diagonals {} and {}", d1, d2),
        Shape::Rectangle(a, b) =>
            println!("Rectangle of edges {} and {}", a, b),
    }
}
```

```
Rectangle of edges 10 and 12
```

---

### Fields in enums can be also named

```rust
enum Shape {
    Circle { radius: f32 },
    Square { edge: f32 },
    Rombus { diag1: f32, diag2: f32 },
    Rectangle { edge1: f32, edge2: f32 },
}
```

---

### Named enums can be destructurized, and fields can be renamed

```rust
fn main() {
    let shape = Shape::Rombus { diag1: 4.0, diag2: 3.0 };
    match shape {
        Shape::Circle { radius } =>
            println!("Circle of radius {}", radius),
        Shape::Square { edge } =>
            println!("Square of edge {}", edge),
        Shape::Rombus { diag1: d1, diag2: d2 } =>
            println!("Rombus of diagonals {} and {}", d1, d2),
        Shape::Rectangle { edge1: a, edge2: b } =>
            println!("Rectangle of edges {} and {}", a, b),
    }
}
```

```
Rombus of diagonals 4 and 3
```

---

### Some fields can be ommited

```rust
fn main() {
    let shape = Shape::Square { edge: 6.0 };
    match shape {
        Shape::Circle { radius } =>
            println!("Circle of radius {}", radius),
        Shape::Square { .. } =>
            println!("Some kind of square"),
        Shape::Rombus { diag1, .. } =>
            println!("Rombus with first diagonal of {}", diag1),
        Shape::Rectangle { edge2, .. } =>
            println!("Rectangle with second edge of {}", edge2),
    }
}
```

```
Some kind of square
```

---

### `if let` can be used to destructurize only single enum variant

```rust
fn main() {
    let shape = Shape::Circle { radius: 1.0 };
    if let Shape::Rombus { .. } = shape {
        println!("Rombus");
    } else {
        println!("Not a rombus");
    }
}
```

```
Not a rombus
```

---

### `while let` allows to loop until something matches pattern

```rust
fn random_shape() -> Shape { /* ... */ }

fn main() {
    while let Shape::Circle { radius } = random_shape() {
        println!("Another circle");
    }

    pritnln!("Not a circle here!");
}
```

---

### `matches!` macro

```rust
fn main() {
    let shape = Shape::Square { edge: 2.0 };
    if matches!(shape, Shape::Square { edge } if edge > 10.0) {
        println!("Big square");
    }
}
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

### `self` can be passed as borrow (simillar to C++ references)

```rust
impl Shape {
    fn corners(&self) -> u8 {
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

---

### Traits as interfaces

```rust
trait Shape {
    fn area(&self) -> f32;
}

struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
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
    fn area(&self) -> f32 {
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

### Traits on foreign types

```rust
trait EuclideanSpace {
    fn length(&self) -> f32;
}

impl EuclideanSpace for Vec<f32> {
    fn length(&self) -> f32 {
        self.iter().map(|x| x * x).sum::<f32>().sqrt()
    }
}
```

---

### Traits on foreign types

```rust
fn main() {
    let v = vec![1.0, 2.0, 3.0];
    println!("Lengh: {}", v.length());
}
```

```
Lengh: 3.7416575
```

---

### Traits methods can have default implementation

```rust
trait Shape {
    fn circumferene(&self) -> f32;
    fn area(&self) -> f32;
    fn circ_to_area_ratio(&self) -> f32 {
        self.circumferene() / self.area()
    }
}
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
    fn dist(&self, right: &Point) -> f32 {
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

