# Learning Rust

## Source :
Hello,

I've found a course that I think you will enjoy.

Author iconUdemy Business
The Rust Programming Language
Learn a modern, powerful yet safe systems programming language!

★★★★☆ 4.30 (3180 ratings)

### Types and Variables
* #### Variables

```rust
use std::mem;
use std::mem::size_of_val;

pub fn variables() {
    /** Variables **/
    // Variable Declaration
    // u = unsigned, 0 to 2^N-1
    // i = signed, -(2^(N-1)) to (2^(N-1))-1
    let a:u8 = 123; // u = unsigned, 8 bits,  in range of [0, 255]
    println!("a={}", a); // immutable variable.

    // a = 456; //  compile error: ^^^^^^^ cannot assign twice to immutable variable
    let mut b: i8 = 0; // m = mutable, i = signed, 8 bits in range of [-128, 127]
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // u8, u16, u32, u64, i8, i16, i32, i64
    // usize, isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let d: char = 'x'; // . ; , #...
    println!("d = {}, takes up {} bytes", d, size_of_val(&d));

    // f32, f64 IEEE754 Signed.
    let e = 2.5;
    println!("e = {}, takes up {} bytes", e, size_of_val(&e));

    let g: bool = false; // false, true
    println!("g = {}, takes up {} bytes", e, size_of_val(&g));
}
```

* #### Operators

```rust
pub fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);

    a = a + 1;
    println!("a = {}", a);

    a -= 2; // -=, +=, *=, /= %=
    println!("a = {}", a);

    println!("remainder of {} / {} = {} ", a, 3, a % 3);

    println!("a^3 = {}", i32::pow(a, 3));

    let b = 2.5;
    println!("b^3 = {}", f64::powi(b, 3));
    println!("b^pi = {}", f64::powf(b, std::f64::consts::PI));

    // bitwise operators
    let c = 1 | 2; // OR operator
    println!("c = {}", c);

    // shift operators >> <<
    println!("2^10 = {}", 1 << 10);

    // logical operator
    println!("Pi Less Than For = {}", std::f64::consts::PI < 4.0);
}
```

* #### Scope and Shadowing

```rust
pub fn scope_and_shadowing() {
    let mut a = 123;
    println!("outer a = {}", a);
    {
        let mut a = 34;
        println!("inner a = {}", a);
        a = 43;
        println!("inner a = {}", a);
    }
    println!("outer a = {}", a);
    a = 33;
    println!("outer a = {}", a);
}
```
* #### Heap and Stack

```rust
struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{
        x: 0.0,
        y: 0.0
    }
}

/**
Heap    -> Heap Dynamic, Allocation and De-allocation
Stack   -> Fast but size is limited. LIFO
 **/
pub fn head_and_stack() {
    let p1 = origin(); // allocated on the stack
    let p2 = Box::new(origin()); // allocated on the heap

    println!("P1 takes up {} bytes on the stack", mem::size_of_val(&p1));
    println!("P2 pointer takes up {} bytes on the heap", mem::size_of_val(&p2));
    println!("P2  takes up {} bytes on the heap", mem::size_of_val(&(*p2)));
}
```


### Control Flow

* #### Decision-Making 

```rust
pub fn if_statement() {
  let temp = 25;

  if temp>30 {
  println!("Temperature is hot");
  } else if temp < 10 {
  println!("Temperature is cold!");
  } else {
  println!("Temperature is nice!")
  }

  let day = if temp > 20 {"Sunny"} else {"Cloudy"};
  println!("Day is {}", day);
  }
```
* #### Loops

```rust
pub fn loops() {
    let mut x = 1;
    while x < 1000 {
        x *=2;
        if x == 64 {
            continue;
        }
        println!("x = {x}");
    }
    let mut y = 1;
    loop { // while true
        y *= 2 ;
        println!("y = {}", y);
        if y == 1 << 10 {break}
    }

    for x in 1..11 { // [1, 10], [1, 11)
        if x == 3 {continue}
        if x == 8 {break}
        println!("x = {x}")
    }

    for (pos, y) in (30..41).enumerate() {
        println!("Index = {pos}, y = {y}")
    }
}
```

* #### Match Statement

```rust
pub fn match_statement() {
    let country_code = 1001;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid" // if we don't cover others, we will get non-exhaustive pattern error!.
    };

    println!("Country Code = {}, Country = {}", country_code, country);
}
```


#### Data Structures

* #### Structures

```rust

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

pub fn structures() {
    let p = Point{x: 3.0, y: 4.0};
    println!("Point p is at ({}, {})", p.x, p.y);

    let p2 = Point{ x: 5.0, y: 10.0};
    let my_line = Line{
        start: p,
        end: p2
    };
    println!(" Line start at ({}, {}) and end at ({}, {})",
             my_line.start.x,
             my_line.start.y,
             my_line.end.x,
             my_line.end.y
    );
}
```

* #### Enums

```rust

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8) // tuple
}

pub fn enums() {
    let c: Color = Color::RgbColor(3, 4, 5);
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({}, {}, {})", r, g, b)
    }
}
```
* #### Unions

```rust

pub union IntOrFloat {
    i: i32,
    f: f32
}

pub fn unions() {
    let iof = IntOrFloat{ i: 42};
    unsafe {
        match iof {
            IntOrFloat {i: 42} => {
                println!("Integer is 42");
            },
            IntOrFloat{ f } => {
                println!("value = {}", f);
            }
        }
    }
}
```

* #### Options

```rust
pub fn options() {
    // Option -> Some(v) | None
    fn process_result(x: f64, y: f64){
        let r = if y != 0.0 { Some(x/y)} else { None };
        match r {
            Some(z) => {
                println!("{x}/{y} = {z}");
            },
            None => {
                println!("{x} can't be divided by {y}!")
            }
        }
    }
    process_result(3.0, 0.0);
    process_result(3.0, 1.0);
}
```

