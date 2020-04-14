# Introduction

* Variables
* Basic of functions declaration
* Control flow
    * `if` conditional statement
    * `while` loop
    * `loop` loop
    * Basic of `for in` loop
* Blocks as expressions
* Printing and formatting

---

### Declaring variables

```rust
let [mut] name [: Type] = initial_value;
```

```rust
let e: f64 = 2.71;
let pi = 3.14;

println!("e: {}, pi: {}", e, pi);
```

```
e: 2.71, pi: 3.14
```

Type is optional, and it can be elided in much more situations than in C++.

---

### Deferred initialization

```rust
let [mut] name [: Type];
name = initial_value;
```

```rust
let number;
if use_pi {
    number = pi;
else {
    number = e;
}

println!("use_pi: {}, number: {}", use_pi, number);
```

```
use_pi: true, number: 3.14
```

---

### Variable has to be initialized on all possible branches before it can be used

```rust
let number;
if use_pi {
    number = pi;
}

println!("number: {}", number);
```

```
error[E0381]: borrow of possibly-uninitialized variable: `number`
  --> src/main.rs:10:28
   |
10 |     println!("Number: {}", number);
   |                            ^^^^^^ use of possibly-uninitialized `number`
```

---

### Basic operations can be performed on variables

```rust
let number = 4;

println!("number + 1 = {}", number + 1);
println!("number - 5 = {}", number - 5);
println!("number * 4 = {}", number * 4);
println!("number / 2 = {}", number / 2);
println!("number % 3 = {}", number % 3);
```

```
number + 1 = 5
number - 5 = -1
number * 4 = 16
number / 2 = 2
number % 3 = 1
```

There are no `++` and `--` operator - just use `+= 1` and `-= 1`.

---

### After variables are initialized they cannot be overwritten by default - constness by default

```rust
let number = pi;
number = e;
```

```
error[E0384]: cannot assign twice to immutable variable `number`
 --> src/main.rs:5:5
  |
4 |     let number = pi;
  |         ------
  |         |
  |         first assignment to `number`
  |         help: make this binding mutable: `mut number`
5 |     number = e;
  |     ^^^^^^^^^^ cannot assign twice to immutable variable
```

---

### Mutability has to be expressed explicitely with `mut` keyword

```rust
let mut number = pi;
number = e;
println!("number: {}", number);
```

```
number: 2.71
```

---

### Variables can be mutated and assigned in place

```rust
let mut number = 2;

number += 1;
number *= 3;
number -= 1;
number /= 2;

println!("number: {}", number);
```

```
number: 4
```

---

### Variables can be shadowed even in the same scope they were declared

```rust
let number = pi;
let number = 2;

println!("number: {}", number);
```

```
number: 2
```

Important note: shadowed variable is *not* considered going out of scope (in particular destructor is not called, and resources are not released).

Note:
Second number has different type - it is not reassignment, it is actually new variable.


---

### Variable type is elided based on assigned expression type, literal type can be described by value suffix

```rust
let pi64 = 3.14f64;
```

---

### Basic types - integral

| Type                    | Meaning                               |
|:------------------------|--------------------------------------:|
| `i8`, `i16`, `i32`, `i64`, `i128` | Fixed size signed integral            |
| `isize`                   | Native pointer size signed integral   |
| `u8`, `u16`, `u32`, `u64`, `u128` | Fixed size unsigned integral          |
| `usize`                   | Native pointer size unsigned integral |

Note:
All integrals are always u2 encoded in Rust!

---

### Basic types - non integral

| Type | Meaning                                  |
|:-----|-----------------------------------------:|
| `bool` | Boolean `true` or `false`                    |
| `char` | UTF-32 encoded character                 |
| `f32`  | IEEE 754 single precision floating point |
| `f64`  | IEEE 754 double precision floating point |
| `()`   | Unit type - type with exactly one value  |

---

### Compound types

| Type         | Meaning                       |
|:-------------|------------------------------:|
| `[T; n]`       | Array of `n` elements of type `T` |
| `[T]`          | Slice of elements of type `T`   |
| `str`          | Slice of chars                |
| `(T1, T2, T3)` | Tuple of types `T1`, `T2`, `T3`     |

Slices are elements sequenced in memory, and their size are unknown in compile time. This has several consequences, for example they cannot be stored on stack.

Note:
In arrays `n` has to be a compile time constant

---

### Basic function declaration

```
fn name(arg1: Arg1Type, arg2: Arg2Type) [-> ReturnType] {
    function body
}
```

Default return type is an unit type (`()`), not C++ `void` equivalent (which is `!` - `never` type which cannot be instantiated - however it is not stable API yet).

---

### `main` function

```rust
fn main() {
    println!("Hello world!");
}
```

In Rust main function doesn't take any arguments. Command line arguments and environment can be accessed via standard library: `std::env::args` and `std::env::vars` functions.

---

### `main` function

```rust
fn main() {
    println!("Hello world!");
}
```
Main function also doesn't return any exit code - to exit application, `std::process::exit` function can be used. However main function can return an `Result` type to report unhandled error.

---

### Example function

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("1 + 3 = {}", add(1, 3));
}
```

```
1 + 3 = 4
```

Last expression is a function result - no `return` statement is needed.

---

### Example function

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}
```

```
error[E0308]: mismatched types
 --> src/main.rs:1:27
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ---                    ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     a + b;
  |          - help: consider removing this semicolon
```

Note missing semicolon at the end - when it is added, expression is turned into a statement, so at the end of function additional implicit expression `()` is added.

---

### Example function - early `return`

```rust
fn number(use_pi: bool) -> f32 {
    if use_pi {
        return 3.14;
    }

    2.71
}

fn main() {
    println!("number: {}", number(true));
}
```

```
number: 3.14
```

---

### Functions cannot be overriden in Rust

```rust
fn add(a: f32, b: f32) -> f32 { a + b }
fn add(a: f64, b: f64) -> f64 { a + b }
```

```
error[E0428]: the name `add` is defined multiple times
 --> src/lib.rs:2:1
  |
1 | fn add(a: f32, b: f32) -> f32 { a + b }
  | ----------------------------- previous definition of the value `add` here
2 | fn add(a: f64, b: f64) -> f64 { a + b }
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `add` redefined here
  |
  = note: `add` must be defined only once in the value namespace of
          this module
```

---

### Function arguments can be mutable too

```rust
fn add(mut a: f32, b: f32) -> f32 {
    a += b;
    a
}

fn main() {
    println!("2 + 5 = {}", add(2.0, 5.0));
}
```

```
2 + 5 = 7.0
```

Mutable argument is *not* an output argument - it is only mutable in scope of the function.

---

### Functions can be declared in scope of another functions

```rust
fn main() {
    fn add(mut a: f32, b: f32) -> f32 {
        a += b;
        a
    }

    println!("2 + 5 = {}", add(2.0, 5.0));
}
```

```
2 + 5 = 7.0
```

---

### `if` conditioanl statement

```rust
if condition {
    ...
} else if other_condition {
    ...
} else {
    ...
}
```

Note missing brackets around condition. Also in contrast to C++, curly braces are always mandatory.

---

### `if` statement is itself an expression which can return value

```rust
let number = if use_pi {
    pi
} else {
    e
};

println!("use_pi: {}, number: {}", use_pi, number);
```

```
use_pi: true, number: 3.14
```

---

### All brances of an `if` statement has to return the same type

```rust
if use_pi {
    pi
} else {
    2
};
```

```
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:9:9
   |
6  | /     if use_pi {
7  | |         pi
   | |         -- expected because of this
8  | |     } else {
9  | |         2
   | |         ^ expected floating-point number, found integer
10 | |     };
   | |_____- `if` and `else` have incompatible types
```

---

### If `else` statement is missing, it is elided to return `()` by default

```rust
if use_pi {
    pi
};
```

```
error[E0317]: `if` may be missing an `else` clause
 --> src/main.rs:6:5
  |
6 | /     if use_pi {
7 | |         pi
  | |         -- found here
8 | |     };
  | |_____^ expected `()`, found floating-point number
  |
  = note: `if` expressions without `else` evaluate to `()`
  = help: consider adding an `else` block that evaluates to the expected type
```

---

### `while` loop

```rust
while condition {
    body
}
```

---

### Factorial with `while`

```rust
let mut n = 5;
let mut fact = 1;

while n > 1 {
    fact *= n;
    n -= 1;
}

println!("fact: {}", fact);
```

```
fact: 120
```

---

### `loop` loop

```rust
loop {
    body
}
```

Loop is a special Rust syntax for defining infinite loop - equivalent of `while true`, or C++ `for(;;)` loop.

---

### Basic `for in` loop

```rust
for element in Iterator {
    body
}
```

Iterator is special trait for iteration in Rust - this will be described in details later, for now just focus on very particular `Iterator` type...

---

### Basic `for _ in range` loop

... range

```rust
for i in 0..5 {
    println!("i: {}", i);
}
```

```
i: 0
i: 1
i: 2
i: 3
i: 4
```

---

### Right side of range is opened by default, but it can be enclosed

```rust
for i in 0..=5 {
}
```

```
i: 0
i: 1
i: 2
i: 3
i: 4
i: 5
```

---

### Factorial with `for _ in range`

```rust
let n = 5;
let mut fact = 1;

for i in 2..=n {
    fact *= i;
}

println!("fact: {}", fact);
```

```
fact: 120
```

---

### `continue` and `break`

```rust
let mut n = 0;
loop {
    if n >= 10 { break; }
    n += 1;
}

while n < 100 {
    if n % 3 == 2 { continue; }
    n += 1;
}
```

`break` and `continue` statements can be used in any kind of loops.

---

### Lifetime labels

```rust
let mut n = 0;
`outer: loop {
    for i in 0..10 {
        n += i;
        if n > 100 {
            break 'outer;
        }
    }
}
```

Lifetime labels can be used to `break` or `continue` outer loop

---

### `loop` as expression

```rust
let mut n = 2;
let mut m = 1;
let fact = loop {
    m *= n;
    n += 1;
    if m > 100 {
        break m;
    }
};
println!("First factorial greater than 100: {}", fact);
```

```
First factorial greater than 100: 120
```

`loop` is also an expression - values is returned from it using `break`, and every `break` has to return value of same type.

---

### Blocks are also expressions

```rust
let res = {
    let a = 4;
    let b = 6;
    a + b
};

pritnln!("res: {}", res);
```

```
res: 10
```

---

### Printing with `print!` and `println!`

```rust
pritnln!("format_string", arg1, arg2);
```

`print!` and `println!` macros allow to format string in similar way as C `printf` function, but with arguments typecheck in compile time. Details about formatting can be found in <https://doc.rust-lang.org/std/fmt/>.

Additionally `eprint!` and `eprintln!` macros are dual to `print!` and `println!`, but prints to `stderr` instead of `stdout`.

---

### Formatting with `format!`

```rust
let msg = format!("format_string", arg1, arg2);
```
`format` macro is very similat to `print!`/`eprint!`, but instead of sending value to stream, it returns it as heap allocated `String`.

---

### Practice

Function calculating fibonacci numbers in four versions:
* Using `while` loop
* Using `loop` loop
* Using `for in` loop
* Recursive
