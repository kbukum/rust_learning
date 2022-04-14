
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