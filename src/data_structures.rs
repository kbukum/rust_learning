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

fn sum_and_product(x: i32, y: i32) -> (i32, i32){
    // tuple construction
    return (x + y, x * y)
}

pub fn tuples() {
    let x1 = 3;
    let y1 = 4;
    // tuple structuring
    let sp1 = sum_and_product(x1, y1);

    println!("sp1({}, {}) = {:?}", x1, y1, sp1);
    println!("{0}+{1} = {2}, {0}*{1} = {3}", x1, y1, sp1.0, sp1.1);

    // destructuring
    let (a1, b1) = sp1;
    println!("{x1}+{y1} = {}, {x1}*{y1} = {}", a1, b1);

    // tuple of tuples
    let x2 = 4;
    let y2 = 7;
    let sp2 = sum_and_product(x2, y2);
    let combined_sp = (sp1, sp2);
    println!("(sp1({}, {}), sp2({}, {})) = {:?}", x1, y1, x2, y2, combined_sp);

    let ((a1,b1), (a2, b2)) = combined_sp;

    println!("{x1}+{y1}={a1}, {x1}*{y1}={b1}, {x2}+{y2}={a2}, {x2}*{y2}={b2}");

    // tuple of different elements
    let tuple_of_different_elements = (true, 42.0, -1i8);
    println!("tuple_of_different_elements ={:?}", tuple_of_different_elements);

    // tuple of single elements
    let tuple_of_single_element = (42, );
    println!("tuple_of_single_element = {:?}", tuple_of_single_element);
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 => "one",
        2 => "couple",
        12 => "dozen",
        3 | 4 => "few",
        z @ 5..=8 => if z % 2 == 0 {"some"} else {"few"},
        _ if (x < 11) => "much",
        _ => "lots of"
    }
}

fn location(p: (i32, i32)) -> String {
    let loc =  match p {
        (0, 0) => format!("origin"),
        (0,y) => format!("x axis, y = {}", y),
        (x,0) => format!("y axis, x = {}", x),
        (x,y) => format!("({}, {})", x, y)
    };
    return loc;
}

fn print_location(p: (i32, i32)) {
    println!("Point is at the {}", location(p));
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} orange{}",x, how_many(x) , if x > 1 {"s"} else {""})
    }

    print_location((0, 0));
    print_location((0, 4));
    print_location((4, 0));
    print_location((4, 4));
}

#[derive(Debug)]
struct DataPoint<T> {
    x: T,
    y: T
}

#[derive(Debug)]
struct DataLine<T> {
    start: DataPoint<T>,
    end: DataPoint<T>
}

pub fn generic_types() {
    let p: DataPoint<i32> = DataPoint{x: 0, y: 1};
    let p1: DataPoint<f64> = DataPoint{x: 0.0, y: 1.0};

    println!("p = {:?}", &p);
    println!("p1 = {:?}", &p1);

    let p2: DataPoint<f64> = DataPoint{x: 0.0, y: 1.0};
    let l = DataLine{ start:p1, end: p2};
    println!("l = {:?}", &l);
}