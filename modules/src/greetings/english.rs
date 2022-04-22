//! This module contains English phareses.
//! ```
//! let username = "John";
//! println!("{} {}", phrases::greetings::english::hello(), username);
//! ```
pub fn hello() -> String { "Hello".to_string() }
pub fn goodbye() -> String { "Good Bye".to_string() }


// Go to folder modules/src from command line
// -> cd modules/src
// -> rustdoc english.rs
