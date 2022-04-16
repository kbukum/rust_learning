
fn print_value(x: i32) {
    println!("value = {}", x);
}

// Borrowing value
fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

pub fn functions_and_arguments() {
    print_value(32);

    let mut z = 1;
    println!("Before change: z = {}", z);
    increase(&mut z);
    println!("After change: z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{}*{} = {}", a, b, p);
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn methods() {
    let p1 = Point{ x: 0.0, y: 3.0 };
    let p2 = Point{ x: 1.0, y: 10.0};
    let l1 = Line { start: p1, end: p2};
    println!("The length for the line, {:?} = {}", l1, l1.len());
}

fn say_hello() {
    println!("Hello, I am used to be a variable function!");
}

pub fn closures() {
    let sh = say_hello;
    sh();

    // Closure function
    let square = |x:f64| -> f64 { x * x };
    let a = 6.0;
    println!("Square of {} is {}", a, square(a));

    let b = 2;
    { // Scope for borrowing
        let pow_of_b = | x | {
            let mut x_pow = x;
            for _ in 2..b {
                x_pow = x_pow * x;
            }
            x_pow
        };
        let x = 3;
        println!("{}^3 + {} = {}", x, b, pow_of_b(x) + b);
    }
    // borrow b back
    println!("Borrow b back as {b}!");

    // T ( Borrowing by value )
    // T& Borrowing by reference
    // &mut & borrowing as mutable reference.

    // Pass by reference
    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

    // Pass by value
    let plus_four = |mut x: i32| {
        x += 4;
        println!("Local value of x = {}", x)
    };
    let f2 = 12;
    plus_four(f2);
    println!("f = {}", f2);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}
// move -> extend the lifetime
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}
/**
functions that take/return(generators) functions
s(x) = f(g(x))
**/
pub fn high_order_functions() {
    //  sum of all even squares < 500
    let limit = 500;
    let mut sum = 0;

    // let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i * i;

        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x) // apply function to each element
        .take_while(|&x| x < limit)
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum + x); //

    println!("High order sum2 = {}", sum2);

}