pub mod greetings {
    // mod grouping functionality
    pub mod english;
    pub mod french {
        pub fn hello() -> String { "Bonjour".to_string() }
        pub fn goodbye() -> String { "Au Revoir".to_string() }
    }
}

#[test]
fn english_greeting_correct() {
    assert_eq!("Hello", greetings::english::hello())
}