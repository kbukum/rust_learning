mod types_and_variables;
mod control_flow;
mod data_structures;
mod collections;
mod characters_and_strings;
mod apps;
mod functions;
mod traits;
mod lifetime_and_memory;

/** Global Variables **/

static  APPLICATION_NAME: &str = "Learning Rust is Fun";

pub fn print_header(header: &str, fn_list: Vec<(&str, fn())>) {
    println!("### {}", header);
    for (sub_header, run) in fn_list {
        println!("#### {}", sub_header);
        run();
        println!("------------------\n");
    }
}


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
fn main() {

    println!("{}", APPLICATION_NAME);

    print_header("Types and Variables", vec![
        ("Variables", types_and_variables::variables),
        ("Operators", types_and_variables::operators),
        ("Scope and Shadowing", types_and_variables::scope_and_shadowing),
        ("Heap and Stack", types_and_variables::head_and_stack)
    ]);

    print_header("Control Flow", vec![
        ("If Statement", control_flow::if_statement),
        ("Loops", control_flow::loops),
        ("Match Statement", control_flow::match_statement)
    ]);

    print_header("Data Structures", vec![
        ("Structures", data_structures::structures),
        ("Enums", data_structures::enums),
        ("Unions", data_structures::unions),
        ("Option<V>", data_structures::options),
        ("Arrays", data_structures::arrays),
        ("Slices", data_structures::slices),
        ("Tuples", data_structures::tuples),
        ("Pattern Matching", data_structures::pattern_matching),
        ("Generic Types", data_structures::generic_types),
    ]);

    print_header("Collections", vec![
        ("Vectors", collections::vectors),
        ("HashMap", collections::hashmap),
        ("HashSet", collections::hashset),
        ("Iterators", collections::iterators)
    ]);

    print_header("Characters and Strings", vec![
        ("Strings", characters_and_strings::strings),
        ("String format", characters_and_strings::string_formats)
    ]);

    print_header("Functions", vec![
        ("Functions and Function Arguments", functions::functions_and_arguments),
        ("Methods", functions::methods),
        ("Closures", functions::closures),
        ("High Order Functions", functions::high_order_functions),
    ]);

    print_header("Traits", vec![
        ("Traits", traits::traits),
        ("Trait Parameters", traits::trait_parameters),
        ("Into Trait", traits::into_trait),
        ("Drop Trait", traits::drop_trait),
        ("Over Loading", traits::overloading),
        ("Static & Dynamic Dispatch", traits::static_dynamic_dispatch),
        ("Vector of different types", traits::vector_of_different_types)
    ]);

    print_header("Lifetime and Memory", vec![
        ("Ownership", lifetime_and_memory::ownership),
        ("Borrowing", lifetime_and_memory::borrowing),
        ("Lifetime", lifetime_and_memory::lifetime),
        ("Lifetime in Structure implementation", lifetime_and_memory::lifetime_in_struct),
        ("Referenced counted variables", lifetime_and_memory::referenced_counted_variables),
        ("Atomic Referenced counted variables", lifetime_and_memory::atomic_referenced_counted_variables),
        ("Mutex Referenced counted variables", lifetime_and_memory::using_mutex_for_thread_safety)
    ]);

    print_header("Applications", vec![
        //("Guessing a number", apps::number_guessing),
        ("Circular References", apps::circular_references),
        ("Spawning and Joining Threads", apps::spawning_and_joining_threads)
    ]);
}
