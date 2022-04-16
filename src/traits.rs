use std::f64::consts::PI;
use std::fmt::Debug;
use std::ops::Add;

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) { // default implementation
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Self {
        return Human{name};
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello!", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Self {
        return Cat{name};
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow!", self.name());
    }
}


trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {
            result += *x;
        }
        return result
    }
}

pub fn traits() {
    let h: Human = Animal::create("John");
    h.talk();
    let c: Cat = Animal::create("Misty");
    c.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    radius: f64
}

// another way to implement trait.
#[derive(Debug, Clone, Copy)]
struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

// implementing trait for the specific struct
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// implementing trait for the specific struct
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
}
// using two traits
fn print_info(shape: impl Shape + Debug) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

// using two traits
fn print_info2<T: Shape + Debug>(shape: T) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

// using two traits
fn print_info3<T>(shape: T) where T:Shape + Debug {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

pub fn trait_parameters() {
    let c = Circle { radius: 2.0};
    print_info(c);
    print_info2(c);
    print_info3(c);
}

struct Person {
    name: String
}

impl Person {
    fn new(name: &str) -> Person {
        Person{name: name.to_string()}
    }
}

struct PersonInto {
    name: String
}

impl PersonInto {
    fn new<S: Into<String>>(name: S) -> PersonInto {
        PersonInto { name: name.into()}
    }
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(unused)]
    fn new2<S>(name: S) -> PersonInto where S:Into<String> {
        PersonInto { name: name.into()}
    }
}

pub fn into_trait() {
    // Into allow to parameter to automatic conversion.
    let person  = Person::new("Kami");

    println!("(Using Embedded String) Person name : {}", person.name);

    let name = "Jane".to_string();
    let p2 = Person::new(name.as_ref());
    println!("(Using reference of string) Person name : {}", p2.name);

    let name3 = "Into Name";
    let p3 = PersonInto::new(name3);
    println!("(Using into generic to copy the string) Person name : {}", p3.name);

    let name4 = "Into Name";
    let p4 = PersonInto::new(name4);
    println!("(Using into where generic to copy the string) Person name : {}", p4.name);
}

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature {name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}
pub fn drop_trait() {
    let creature = Creature::new("Test");
    println!("Creature Name: {}", creature.name);
    println!("Drop is called");
    drop(creature);
}

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> {re, im}
    }
}

impl <T> Add for Complex<T>
where T: Add<Output = T>
{
    type Output = Complex<T>;
    // a + b
    fn add(self, rhs: Self) -> Self::Output {
        Complex{ re: self.re + rhs.re, im: self.im + rhs.im}
    }
}

pub fn overloading() {
    let a = Complex::new(1, 2);
    let b = Complex::new(3, 4);

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("a + b = {:?}", a + b);
}