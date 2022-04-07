mod basics;

/** Global Variables **/

static  APPLICATION_NAME: &str = "Learning Rust is Fun";

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
fn main() {
    println!("{}", APPLICATION_NAME);
    println!("###### Variables #######");
    basics::variables();
    println!("------------------\n");
    println!("###### Operators #######");
    basics::operators();
    println!("------------------\n");
    println!("###### Scope and Shadowing #######");
    basics::scope_and_shadowing();
    println!("------------------\n");
    println!("###### Heap and Stack #######");
    basics::head_and_stack();
    println!("------------------\n");
    println!("###### If Statement #######");
    basics::if_statement();
    println!("------------------\n");
    println!("###### Loops #######");
    basics::loops();
    println!("------------------\n");
    println!("###### Match Statement #######");
    basics::match_statement();
    println!("------------------\n");
    println!("###### Structures #######");
    basics::structures();
    println!("------------------\n");
    println!("###### Enums #######");
    basics::enums();
    println!("------------------\n");
    println!("###### Unions #######");
    basics::unions();
    println!("------------------\n");
    println!("###### Option<V> #######");
    basics::options();
    println!("------------------\n");
    println!("###### Arrays #######");
    basics::arrays();
    println!("------------------\n");
}
