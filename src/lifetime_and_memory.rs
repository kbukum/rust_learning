use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
/**
Retrieved from <a>https://doc.rust-lang.org/rust-by-example/scope/move.html</a>
Because variables are in charge of freeing their own resources, resources can only have one owner.
This also prevents resources from being freed more than once. Note that not all variables own resources.

When doing assignments (let x = y) or passing function arguments by value (foo(x)),
the ownership of the resources is transferred. In Rust-speak, this is known as a move.

After moving resources, the previous owner can no longer be used.
This avoids creating dangling pointers.
**/
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn ownership() {
    // Variable v on the stack and owns the vector on the heap
    let v = vec![1, 2, 3];

    // move v to v2 and v is no usable anymore.
    /**
    let v2 = v; // It is copying the pointer.
    println!("{:?}", v); // error:  ^ value borrowed here after move
    **/

    let foo = |v:Vec<i32>| ();
    // v borrowed by foo function
    foo(v);
    // v can't be used since it is borrowed by foo function
    // println!("{:?}", v); // error: value borrowed here after move

    // For primitives there is no problem to borrowing since it is copying the stack value.
    let u = 1;
    let u2 = u;
    println!("u = {}", u);

    let v2 = vec![1, 2, 3];

    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        return x;
    };
    let v2v = print_vector(v2);
    println!("{}", v2v[0]);
}

/**
Retrieved from <a>https://doc.rust-lang.org/rust-by-example/scope/borrow.html</a>
Most of the time, we'd like to access data without taking ownership over it. To accomplish this,
Rust uses a borrowing mechanism. Instead of passing objects by value (T), objects can be passed by reference (&T).

The compiler statically guarantees (via its borrow checker) that references always point to valid objects.
That is, while references to an object exist, the object cannot be destroyed.
**/
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn borrowing() {
    let print_vector = |x:&Vec<i32>| {
        println!("{:?}", x);
        // x.push(123) -> error : immutable reference.
    };

    let v = vec![3, 2, 1];
    // borrowing by using &
    print_vector(&v);
    println!("{:?}", v); // can be used since it is only borrowed.

    let mut a = 40;
    let b = &mut a;
    *b += 2;
    println!("a = {}", a);
    // only mutable variable can be borrowed as mutable.

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!(" i = {}", i);
        // error -> z.push(4); mitable borrow occurs here.
    }
}

#[derive(Debug)]
struct Person {
    name: String
}

impl Person {
    /**
    Elide lifetime
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
    The following function is same as the above function.
    **/
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
struct Company<'a> { // Lifetime of Person should be same as Company.
    name: String,
    ceo: &'a Person // we need to define the lifetime for ceo when company is created
}

/**
Retrieved from <a>https://doc.rust-lang.org/rust-by-example/scope/lifetime.html</a>
A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid.
Specifically, a variable's lifetime begins when it is created and ends when it is destroyed.
While lifetimes and scopes are often referred to together, they are not the same.

Take, for example, the case where we borrow a variable via &. The borrow has a lifetime that is determined by where it is declared.
As a result, the borrow is valid as long as it ends before the lender is destroyed. However, the scope of the borrow is determined
by where the reference is used.

 **/
// &'static str -> static mean variable live as long as program lives.
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn lifetime() {
    let boss = Person{name: String::from("Elon Musk")};
    let tesla = Company{name: String::from("Elon Musk"), ceo: &boss}; // how long boss should be valid!
    println!("{:?}", tesla);

    let mut z: &String;
    {
        let p = Person {name: String::from("John")};
        z = p.get_ref_name();
    }
}

#[derive(Debug)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
struct Person2<'a>{
    name: &'a str
    // name: &str wrong lifetime parameter
}

impl<'b> Person2<'b> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn lifetime_in_struct() {
    let p = Person2{ name: "John" };
    println!("p = {:?}", p);
    p.talk();
}

struct Person3{
    name: Rc<String>
}

impl Person3 {
    fn new(name: Rc<String>) -> Person3 {
        Person3{name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn referenced_counted_variables() {
    let name = Rc::new("John".to_string());
    // moving name
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person3::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
}

struct Person4{
    name: Arc<String>
}

impl Person4 {
    fn new(name: Arc<String>) -> Person4 {
        Person4{name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn atomic_referenced_counted_variables() {
    /**
     |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
    Rc not thread safe
     {
        let name = Rc::new("John".to_string());

        let person = Person3::new(name);
        let t = thread::spawn(move || {
            person.greet();
        });
        println!("Name = {}", person.name);
    }
    **/

    let name = Arc::new("John".to_string());

    let person = Person4::new(name.clone());
    /**
     |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
    Rc not thread safe
    **/
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);
    t.join().unwrap();
}

struct Person5{
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person5 {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person5 {
        Person5{name, state }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I am very {}", self.name, state);
    }
}

/**
Retrieved from <a>https://doc.rust-lang.org/std/sync/struct.Mutex.html</a>
A mutual exclusion primitive useful for protecting shared data

This mutex will block threads waiting for the lock to become available.
The mutex can also be statically initialized or created via a new constructor.
Each mutex has a type parameter which represents the data that it is protecting.
The data can only be accessed through the RAII guards returned from lock and try_lock,
which guarantees that the data is only ever accessed when the mutex is locked.
**/
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn using_mutex_for_thread_safety() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("Bored".to_string()));
    let person = Person5::new(name.clone(), state.clone());
    /**
     |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
    Rc not thread safe
    **/
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, State = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();
}
