extern crate phrases;

use rand::{Rng};
use std::io::stdin;
use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time};
use phrases::greetings::english;
use phrases::greetings::french;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn number_guessing() {
    let number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Enter your guess:");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100  {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is low")
                        } else if guess > number {
                            println!("Your guess is high")
                        } else {
                            println!("Correct!!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Error occured! {}, Try again!", e);
                    }
                }
            },
            Err(_) => continue
        }
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student {
    fn new(name: String) -> Student {
        Student {
            name,
            courses: Vec::new()
        }
    }
}

#[derive(Debug)]
struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}


impl Course {
    fn new(name: String) -> Course {
        Course {
            name,
            students: Vec::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<Course>>,
        student: Rc<RefCell<Student>>
    ) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student.clone());
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn circular_references() {
    let mut john = Rc::new(
        RefCell::new(
            Student::new("John".to_string())
        )
    );

    let mut course = Rc::new(
        RefCell::new(
            Course::new("Rust Course".to_string())
        )
    );

    Course::add_student(course.clone(), john);

    println!("Course Name: {:?}", course.borrow_mut().name);


    for  (pos, student) in course.borrow_mut().students.iter().enumerate() {
        let s = student.as_ref();
        println!("Student [{}] = {}", pos, s.try_borrow().unwrap().name);
    }
}


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn spawning_and_joining_threads() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(100));
        }
    });
    let handle2 = thread::spawn(|| {
        for _ in 1..10 {
            print!("-");
            thread::sleep(time::Duration::from_millis(200));
        }
    });

    let handle3 = thread::spawn(|| {
        for _ in 1..10 {
            print!("*");
            thread::sleep(time::Duration::from_millis(300));
        }
    });

    let handle4 = thread::spawn(|| {
        for _ in 1..10 {
            print!(":");
            thread::sleep(time::Duration::from_millis(400));
        }
    });

    handle.join();
    handle2.join();
    handle3.join();
    handle4.join();
}

pub fn play_with_modules() {
    println!("{}", english::hello());
    println!("{}", english::goodbye());
    println!("{}", french::hello());
    println!("{}", french::goodbye());
}