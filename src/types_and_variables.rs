use std::mem;
use std::mem::size_of_val;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
/**

 **/
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

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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
