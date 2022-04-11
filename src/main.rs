mod types_and_variables;
mod control_flow;
mod data_structures;

/** Global Variables **/

static  APPLICATION_NAME: &str = "Learning Rust is Fun";

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
fn main() {
    println!("{}", APPLICATION_NAME);
    println!("###### Variables #######");
    types_and_variables::variables();
    println!("------------------\n");
    println!("###### Operators #######");
    types_and_variables::operators();
    println!("------------------\n");
    println!("###### Scope and Shadowing #######");
    types_and_variables::scope_and_shadowing();
    println!("------------------\n");
    println!("###### Heap and Stack #######");
    types_and_variables::head_and_stack();
    println!("------------------\n");
    println!("###### If Statement #######");
    control_flow::if_statement();
    println!("------------------\n");
    println!("###### Loops #######");
    control_flow::loops();
    println!("------------------\n");
    println!("###### Match Statement #######");
    control_flow::match_statement();
    println!("------------------\n");
    println!("###### Structures #######");
    data_structures::structures();
    println!("------------------\n");
    println!("###### Enums #######");
    data_structures::enums();
    println!("------------------\n");
    println!("###### Unions #######");
    data_structures::unions();
    println!("------------------\n");
    println!("###### Option<V> #######");
    data_structures::options();
    println!("------------------\n");
    println!("###### Arrays #######");
    data_structures::arrays();
    println!("------------------\n");
    println!("###### Slices #######");
    data_structures::slices();
    println!("------------------\n");
    println!("###### Tuples #######");
    data_structures::tuples();
    println!("------------------\n");
    println!("###### Pattern Matching #######");
    data_structures::pattern_matching();
    println!("------------------\n");
    println!("###### Generic Types #######");
    data_structures::generic_types();
    println!("------------------\n");
}
