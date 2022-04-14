pub fn strings() {
    // &'static str -> &str = string slice, static -> it is going to be part of the program.
    let s: &'static str = "hello";
    // error: s  = "abc" -> can't be manipulated!
    // error let h = s[0] -> characters can't be retrieved from string array directly.

    for (pos, c) in s.chars().enumerate() {
        println!("s[{}] = {}", pos, c)
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("First Letter is {}", first_char);
    }

    // heap String construct
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("Letters = {}", letters);

    // Conversion between &str and String

    let u: &str = &letters;
    println!("&str Letters = {}", &u);

    // concatenation
    // String + str
    //let z = letters + "abc";
    // println!("concatenated Letters and abc = {}", z);
    let letters_and_letters = letters.to_owned() + &letters;
    println!("concatenating letters and letters = {}", letters_and_letters);

    // String from str
    let mut abc = String::from("abc");
    println!("abc = {}", abc);
    abc = "def".to_string();
    println!("abc = {}", abc);
    abc.remove(0);
    println!("abc = {}", abc);
    abc.push_str("!!!");
    println!("abc = {}", abc);
}

pub fn string_formats() {
    let name = "Kamil";
    let greeting = format!("hi, I'm {}, nice to meet you. ", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0} {1} {0}", run, forest);
    println!("{}", rfr);

    let info = format!("{first} {last}",
                       first = "Hello",
                       last = "World"
    );
    println!("{}", info);
}


