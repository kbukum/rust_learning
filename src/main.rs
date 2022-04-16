mod types_and_variables;
mod control_flow;
mod data_structures;
mod collections;
mod characters_and_strings;
mod apps;
mod functions;
mod traits;

/** Global Variables **/

static  APPLICATION_NAME: &str = "Learning Rust is Fun";

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
fn main() {
    println!("{}", APPLICATION_NAME);

    println!("\n###### Types and Variables #######\n");

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

    println!("\n###### Control Flow #######\n");

    println!("###### If Statement #######");
    control_flow::if_statement();
    println!("------------------\n");
    println!("###### Loops #######");
    control_flow::loops();
    println!("------------------\n");
    println!("###### Match Statement #######");
    control_flow::match_statement();
    println!("------------------\n");



    println!("\n###### Data Structures #######\n");

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


    println!("\n###### Collections #######\n");

    println!("###### Vectors #######");
    collections::vectors();
    println!("------------------\n");
    println!("###### HashMap #######");
    collections::hashmap();
    println!("------------------\n");
    println!("###### HashSet #######");
    collections::hashset();
    println!("------------------\n");
    println!("###### Iterators #######");
    collections::iterators();
    println!("------------------\n");

    println!("\n###### Characters and Strings #######\n");
    println!("###### Strings #######");
    characters_and_strings::strings();
    println!("------------------\n");
    println!("###### String format #######");
    characters_and_strings::string_formats();
    println!("------------------\n");


    println!("\n###### Functions #######\n");
    println!("###### Functions and Function Arguments #######");
    functions::functions_and_arguments();
    println!("------------------\n");
    println!("###### Methods #######");
    functions::methods();
    println!("------------------\n");
    println!("###### Closures #######");
    functions::closures();
    println!("------------------\n");
    println!("###### High Order Functions #######");
    functions::high_order_functions();
    println!("------------------\n");

    println!("\n###### Traits #######\n");
    println!("###### Traits #######");
    traits::traits();
    println!("------------------\n");
    println!("###### Trait Parameters #######");
    traits::trait_parameters();
    println!("------------------\n");
    println!("###### Into Trait #######");
    traits::into_trait();
    println!("------------------\n");
    println!("###### Drop Trait #######");
    traits::drop_trait();
    println!("------------------\n");
    println!("###### Over Loading #######");
    traits::overloading();
    println!("------------------\n");

    println!("\n###### Applications #######\n");
    println!("###### Guessing a number #######");
    // apps::number_guessing();
    println!("------------------\n");
}
