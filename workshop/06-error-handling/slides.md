## Closures, error handling and iterators

---

### Closures

```rust
[move] |arguments| [-> ReturnType] {
    // Closure body
}
```

---

### Simple closure

```rust
fn main() {
    let printer = || {
        println!("Hello closures!");
    };

    printer();
}
```

```
Hello closures!
```

---

### Closure returning a value

```rust
fn main() {
    let answer = || 41;
    println!("Answer about life, universe and everything else: {}", answer());
}
```

```
Answer about life, universe and everything else: 41
```

---

### Closure taking arguments

```rust
fn main() {
    let printer = |answer| {
        println!("Answer about life, universe and everything else: {}", answer)
    };

    printer(41);
}
```

```
Answer about life, universe and everything else: 41
```

---

### Capturing context

```rust
fn main() {
    let circle = Circle(4.0);
    let closure = || {
        println!("Area: {}", circle.area());
    };
    println!("Circumference: {}", circle.circumference());
    closure();
}
```

```
Circumference: 25.132742
Area: 50.265484
```

---

### Capturing context by `move`

```rust
fn main() {
    let circle = Circle(4.0);
    let closure = move || {
        println!("Area: {}", circle.area());
    };
    println!("Circumference: {}", circle.circumference());
    closure();
}
```

```
error[E0382]: borrow of moved value: `circle`
  --> src/main.rs:18:35
   |
15 |     let closure = move || {
   |                   ------- value moved into closure here
16 |         println!("Area: {}", circle.area());
   |                              ------ variable moved due to use in closure
17 |     };
18 |     println!("Circumference: {}", circle.circumference());
   |                                   ^^^^^^ value borrowed here after move
```

---

### Combining move and borrow capture

```rust
fn main() {
    let circle1 = Circle(4.0);
    let circle2 = Circle(5.0);
    let closure = {
        let circle2 = &circle2;
        move || {
            println!("Areas: {}, {}", circle1.area(), circle2.area());
        }
    }
    println!(
        "Circumferences: {}, {}",
        circle1.circumference, circle2.circumference,
    );
}
```

---

### Function traits

#### FnOnce -> FnMut -> Fn

* `FnOnce(u32) -> f32`
* `FnMut(String) -> String`
* `Fn(&Circle) -> f64`

---

### Returning a one-shot closure

```rust
fn circle_maker(radius: f32) -> impl FnOnce() -> Circle {
    let circle = Circle(radius);
    move || circle
}

fn main() {
    println!("Area: {}", circle_maker(1.0)().area());
}
```

```
Area: 3.1415927
```

---

### Returning statefull closure

```rust
fn growing_circle() -> impl FnMut(f32) -> Circle {
    let mut radius = 0.0;
    move |increment| {
        radius += increment;
        Circle(radius)
    }
}

fn main() {
    let mut generator = growing_circle();
    println!("Areas: {}, {}, {}",
        generator(1.0).area(), generator(1.2).area(), generator(1.6).area()
    );
}
```

```
Areas: 3.1415927, 15.20531, 45.364605
```

---

### Returning pure(ish) closure

```rust
fn adder<T>(a: T) -> impl Fn(T) -> T
where
    T: Copy + std::ops::Add<Output = T>,
{
    move |b| a + b
}

fn main() {
    let add2 = adder(2.0);
    println!("Add2: {}, {}, {}", 0.5, 1.2, 4.0);
}
```

```
Add2: 0.5, 1.2, 4
```

---

### Returning optional value with `Option<T>` type

```rust
enum Option<T> {
    Some(T),
    None,
}

fn ident_if_less(num: f32, limit: f32) -> Option<f32> {
    if num < limit {
        Some(num)
    } else {
        None
    }
}
```

---

### Optional argument with `Option<T>` type

```rust
fn number_or_default(num: Option<f32>, def: f32) -> f32 {
    if let Some(num) = num {
        num
    } else {
        def
    }
}
```

---

### Optional argument with `impl Into<Option<T>>` trick

```rust
fn number_or_default(num: impl Into<Option<f32>>, def: f32) -> f32 {
    if let Some(num) = num.into() {
        num
    } else {
        def
    }
}

fn main() {
    println!("Number: {}", number_or_default(4.0, 6.0));
    println!("Number: {}", number_or_default(Some(4.0), 6.0));
    println!("Number: {}", number_or_default(None, 6.0));
}
```

```
Number: 4
Number: 4
Number: 6
```

---

### `None` forwarding with `?`

```rust
fn area(radius: impl Into<Option<f32>>) -> Option<f32> {
    Circle(radius.into()?).area().into()
}

fn main() {
    println!("Areas: {:?}, {:?}", area(5.0), area(None));
}
```

```
Areas: Some(78.53982), None
```

---

### Inspecting `Option`

```rust
fn main() {
    let (o1, o2): (_, Option<f32>) = (Some(2.0), None);
    println!("is_some: {}, is_none: {}", o1.is_some(), o1.is_none());
    println!("is_some: {}, is_none: {}", o2.is_some(), o2.is_none());
}
```

```
is_some: true, is_none: false
is_some: false, is_none: true
```

---

### Panicking on `None`

```rust
fn area(radius: Option<f32>) -> f32 {
    Circle(radius.unwrap()).area()
    // Circle(radius.expect("Error message")).area()
}

fn main() {
    area(None);
}
```

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value',
src/main.rs:14:12
```

---

### Extracting value with default

```rust
fn area(radius: Option<f32>) -> f32 {
    Circle(radius.unwrap_or(0.0)).area()
}

fn main() {
    println!("Area: {}", area(None))
}
```

```
Area: 0
```

See also: `unwrap_or_else` for lazy default creation

---

### Mapping and filtering

```rust
fn main() {
    let number = dbg!(Some(3.0));
    let number = dbg!(number.map(|x| x * x));
    let number = dbg!(number.filter(|x| *x < 5.0));
    let number = dbg!(number.map(|x| x * x));
}
```

```
[src/main.rs:14] Some(3.0) = Some(
    3.0,
)
[src/main.rs:15] number.map(|x| x * x) = Some(
    9.0,
)
[src/main.rs:16] number.filter(|x| *x < 5.0) = None
[src/main.rs:17] number.map(|x| x * x) = None
```

---

### Mapping and filtering

* `map_or` - maps with closure, or returns default value
* `map_or_else` - maps with closure, or return default value created by other closure
* `and` - for `None` returns `None`, for `Some` returns another given `Option`
* `and_then` - like `map`, but closure returns `Option` (known as flat_map or monadic map)
* `or`, `or_else`, `xor`

---

### Converting to borrow

```rust
fn main() {
    let mut text = Some("Hello".to_owned());
    let len = text.as_ref().map(|text| text.len());
    if let Some(text) = text.as_mut() {
        text.make_ascii_uppercase();
    }
    println!("Text: {:?}, len: {:?}", text, len);
}
```

```
Text: Some("HELLO"), len: Some(5)
```

---

### Returnign error with `Result<T, E>` type

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn ident_if_less(num: f32, limit: f32) -> Result<f32, String> {
    if num < limit {
        Ok(num)
    } else {
        Err("Numbers is over its limit")
    }
}
```
---

### Error case forwarding with `?`

```rust
fn area(radius: Result<f32, String>) -> Result<f32, String> {
    Ok(Circle(radius?).area())
}

fn main() {
    println!("Areas: {:?}, {:?}",
        area(Ok(5.0)), area(Err("Invalid radius".to_owned()))
    );
}
```

```
Areas: Ok(78.53982), Err("Invalid radius")
```

---

### Error type can differ when `?` forwarding - proper `Into` trait is enaugh

```rust
fn area(radius: Result<f32, &str>) -> Result<f32, String> {
    Ok(Circle(radius?).area())
}

fn main() {
    println!("Areas: {:?}, {:?}",
        area(Ok(5.0)), area(Err("Invalid radius"))
    );
}
```

```
Areas: Ok(78.53982), Err("Invalid radius")
```

---

### `Result` can be inspected just like `Option`

```rust
fn main() {
    let (o1, o2): (Result<_, f32>, Result<f32, _>) = (Ok(2.0), Err(3.0));
    println!("is_ok: {}, is_err: {}", o1.is_ok(), o1.is_err());
    println!("is_ok: {}, is_err: {}", o2.is_ok(), o2.is_err());
}
```

```
is_ok: true, is_err: false
is_ok: false, is_err: true
```

---

### `unwrap` and `expect` also works

```rust
fn area(radius: Result<f32, &str>) -> f32 {
    Circle(radius.unwrap()).area()
    // Circle(radius.expect("Error message")).area()
}

fn main() {
    area(Err("Invalid radius"));
}
```

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err`
value: "Invalid radius"', src/main.rs:14:12
```

---

### `unwrap_or` and `unwrap_or_else` too

```rust
fn area(radius: Result<f32, &str>) -> f32 {
    Circle(radius.unwrap_or(0.0)).area()
}

fn main() {
    println!("Area: {}", area(Err("Invalid radius")))
}
```

```
Area: 0
```

---

### Mapping set of function is very much similar

* `map`, `map_or` - as in `Option`
* `map_or_else` - else closure takes an `Err` case type
* `and`, `and_then` - just like in `Option`

---

### But what is important - it is possible to map error only

```rust
fn area(radius: Result<f32, u32>) -> Result<f32, String> {
    let radius = radius
        .map_err(|code| format!("Radius error with code: {}", code))?;
    Ok(Circle(radius).area())
}

fn main() {
    println!("Area: {:?}", area(Err(5)));
}
```

```
Area: Err("Radius error with code: 5")
```

---

### Important thing about `Result` type is that it cannot be just ignored without warning

```rust
fn print_inverse(num: f32) -> Result<(), String> {
    if num == 0.0 {
        Err("Cannod divide by 0!".to_owned())
    } else {
        Ok(println!("{}", 1.0 / num))
    }
}

fn main() {
    print_inverse(4.0);
}
```

---

### Important thing about `Result` type is that it cannot be just ignored without warning

```
warning: unused `std::result::Result` that must be used
  --> src/main.rs:22:5
   |
22 |     print_inverse(4.0);
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled
```

---

### To ignore it explicitely we can convert it to `Option`

```rust
fn print_inverse(num: f32) -> Result<(), String> {
    if num == 0.0 {
        Err("Cannod divide by 0!".to_owned())
    } else {
        Ok(println!("{}", 1.0 / num))
    }
}

fn main() {
    print_inverse(4.0).ok();
}
```

This converts `Result<T, U>` to `Option<T>`. `Result::err` can be used to convert to `Option<U>`.

---

### `Result` can be created from `Option` by mapping `None` to error type

```rust
fn area(radius: impl Into<Option<f32>>) -> Result<f32, String> {
    let radius = radius.into().ok_or("Empty radius".to_owned())?;
    Ok(Circle(radius).area())
}

fn main() {
    println!("Area: {:?}", area(None));
}
```

```
Area: Err("Empty radius")
```

`ok_or_else` can be used to create error lazely.

---

### `Result<Option<T>, E>` <=> `Option<Result<T, E>>` can be easly converted

```rust
fn main() {
    let opt_res: Option<Result<u32, _>> = dbg!(Some(Err(2)));
    let res_opt = dbg!(opt_res.transpose());
    let opt_res = dbg!(res_opt.transpose());
}
```

---

### `Result<Option<T>, E>` <=> `Option<Result<T, E>>` can be easly converted

```
[src/main.rs:14] Some(Err(2)) = Some(
    Err(
        2,
    ),
)
[src/main.rs:15] opt_res.transpose() = Err(
    2,
)
[src/main.rs:16] res_opt.transpose() = Some(
    Err(
        2,
    ),
)
```

---

### Creating own error type

```rust
#[derive(Debug)]
enum Error {
    IOError(std::io::Error),
    LengthMissmatch(usize),
    OtherError(String),
}
```

But it is feature poor... it is hard to use it convienently.

---

### We can use crate like `thiserror`

```rust
#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Error while performing IO operation: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Expected at least {0} more bytes, but stream ended")],
    LengthMissmatch(usize),
    #[error("Unexpected error: {0}")]
    OtherError(String),
}
```

---

### Another crate, when we don't want to deal with actuall error type is `anyhow`

```rust
fn process(input: &[u8]) -> Result<f32, anyhow::Error> {
    // This works for any error types returned by stages, as long as they
    // implement `std::error::Error` trait which most of errors does
    // (thiserror generated errors in particular does)
    let (intermediate, context) = process_stage1(input)?;
    let data = process_stage2(context)?;
    Ok(process_stage3(intermediate, data)?)
}
```

---

### `thiserror` and `anyhow` cooperates nicely

```rust
#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Error while performing IO operation: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Expected at least {0} more bytes, but stream ended")],
    LengthMissmatch(usize),
    #[error("Unexpected error: {0}")]
    OtherError(anyhow::Error),
}
```

---

### `Iterator` trait

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Item>;
}
```

If there is valid next item, `next` returns `Some` with an item, `None` means that iteration is finished.

---

### Handmade range iterator

```rust
struct Range(usize, usize);

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.0 < self.1 {
            self.0 += 1;
            Some(self.0 - 1)
        } else {
            None
        }
    }
}
```

---

### Handmade range iterator

```rust
fn main() {
    for i in Range(2, 6) {
        println!("Item: {}", i);
    }
}
```

```
Item: 2
Item: 3
Item: 4
Item: 5
```

---

# Even if `Iterator` requires only one function, it deliver whole bunch of them

---

### Collecting to container

```rust
fn main() {
    let numbers: Vec<_> = (0..10).collect();
    println!("Numbers: {:?}", numbers);
}
```

```
Numbers: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

---

### Mapping and filtering elements

```rust
fn main() {
    let numbers = 1..=20;
    let numbers = numbers.filter(|x| x % 2 == 1);
    let numbers = numbers.map(|x| x * x);
    let numbers: Vec<_> = numbers.collect();
    println!("Numbers: {:?}", numbers);
}
```

```
Numbers: [1, 9, 25, 49, 81, 121, 169, 225, 289, 361]
```

---

### Filter-map can be done one one take

```rust
fn main() {
    let numbers: Vec<_> = (1..=20).filter_map(|x| if x % 2 == 0 {
        None
    } else {
        Some(x * x)
    }).collect();
    println!("Numbers: {:?}", numbers);
}
```

```
Numbers: [1, 9, 25, 49, 81, 121, 169, 225, 289, 361]
```

---

### There are dosens of usefull functions for transfroming iterator

* `count`, `last`, `first`, `nth`
* `step_by`, `skip`, `take`
* `chain`, `zip`, `unzip`, `enumerate`
* `skip_while`, `take_while`, `map_while`
* `fold`, `scan`
* And more

---

### `Iterator` as trait can be used to perform lazy operations

```rust
fn odd_squares(numbers: impl Iterator<Item=u32>) -> impl Iterator<Item=u32> {
    numbers.filter(|x| x % 2 == 1).map(|x| x * x)
}

fn main() {
    let squares = odd_squares(1..=20);
    let halfs: Vec<_> = squares.map(|x| x / 2).collect();
    println!("Result: {:?}", halfs);
}
```

```
Result: [0, 4, 12, 24, 40, 60, 84, 112, 144, 180]
```

---

### We can use `peekable` if we want ability to check top element without eating it

```rust
fn main() {
    let mut numbers = (1..=20).peekable();
    while matches!(numbers.peek(), Some(x) if *x < 5) {
        let x = numbers.next().unwrap();
        println!("{}", x * x);
    }
    let rest: Vec<_> = numbers.collect();
    println!("Unconsumed: {:?}", rest);
}
```

```
1
4
9
16
Unconsumed: [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
```

---

### `Option` and `Result` can be turned into one element iterator

```rust
fn main() {
    for item in Some(5) {
        println!("Item: {}", item);
    }

    let res: Result<_, u32> = Ok(10);
    for item in res {
        println!("Item: {}", item);
    }
}
```

```
Item: 5
Item: 10
```

---

### `Iterator` over `Option` can be collected to `Option` to break on first failure

```rust
fn main() {
    let numbers = vec![Some(5), Some(10), None, Some(12)].into_iter();
    let numbers: Option<Vec<_>> = numbers.collect();
    println!("Numbers: {:?}", numbers);

    let numbers = vec![Some(5), Some(10), Some(3), Some(12)].into_iter();
    let numbers: Option<Vec<_>> = numbers.collect();
    println!("Numbers: {:?}", numbers);
}
```

```
Numbers: None
Numbers: Some([5, 10, 3, 12])
```

---

### Same can be done with `Result`

```rust
fn main() {
    let numbers = vec![Ok(5), Ok(10), Err("Invalid"), Ok(12)].into_iter();
    let numbers: Result<Vec<_>, &'static str> = numbers.collect();
    println!("Numbers: {:?}", numbers);

    let numbers = vec![Ok(5), Ok(10), Ok(3), Ok(12)].into_iter();
    let numbers: Result<Vec<_>, &'static str> = numbers.collect();
    println!("Numbers: {:?}", numbers);
}
```

```
Numbers: Err("Invalid")
Numbers: Ok([5, 10, 3, 12])
```

---

### `std::iter::IntoIterator` trait

```rust
trait IntoIterator
where
    <Self::IntoIter as Iterator>::Item == Self::Item,
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

---

### `std::iter::IntoIterator` trait

`IntoIterator` trait is like mariage of `Into` and `Iterator` - it is defined for types which
can be turned into an `Iterator`. It is defined for every container, but also for
`Option` and `Result`, and every `Iterator` itself (performing identity conversion). This
is universal way of taking iterable types. Anything implementing `IntoIterator` can
be used in `for _ in _` loop.
