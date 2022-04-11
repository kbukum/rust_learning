use std::mem;

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8) // tuple
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub union IntOrFloat {
    i: i32,
    f: f32
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
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

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn arrays() {
    let mut a:[i32;5] = [1, 2,3, 4, 5];
    println!("a has {} elements, first element is {}", a.len(), a[0]);
    a[0] = 321;
    println!("a has {} elements, first element is {}", a.len(), a[0]);
    // :? debug printing
    println!("a = {:?}", a);

    if a != [1,2,3,4,5] {
        println!("Array doesnt match!");
    }

    // create an array with length 10 and set all values to 1
    let b = [1; 10];
    println!("b has {} elements and takes up {} bytes", b[0], mem::size_of_val(&b));

    for i in 0..a.len() {
        println!("Pos_{} = {}", i, a[i])
    }

    // create an array with length 10 and type u16 and set all values to 1
    let c = [1u16; 10];
    println!("c has {} elements and takes up {} bytes", c[0], mem::size_of_val(&c));

    // multi dimensional arrays
    let mtx:[[f32; 3];2] = [
        [1.0, 2.0, 3.0],
        [1.5, 2.5, 3.5]
    ];
    println!("mtx has {} elements and takes up {} bytes", mtx[0][0], mem::size_of_val(&mtx));

    for i in 0..mtx.len() {
        for j in 0..mtx.len() {
            if i == j  {
                println!("mtx[{i}][{j}] = {}", mtx[i][j])
            }
        }
    }
}

fn use_slice(slice: &mut [i32]) {
    println!("Sliced array = {:?}, first element is {}" ,&slice, slice[0]);
    slice[0] = 5;
    println!("Sliced array = {:?}, first element replaced to {}" ,&slice, slice[0]);
}

pub fn slices() {
    let mut data = [1,2 ,3 ,4, 5];
    println!("Before Change Original Array = {:?}" ,&data);
    use_slice(&mut data[1..4]);
    println!("After Replacement on Slice the Original array = {:?}" ,&data);
}

pub fn tuples() {

}