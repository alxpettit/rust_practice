#[allow(dead_code)]
mod arrays;
mod basic_class;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuple_structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    // Index here counts from 1
    // Set number according to which example you want to run...
    let example_num = 16;

    match example_num {
        1 => print::main(),
        2 => vars::main(),
        3 => types::main(),
        4 => strings::main(),
        5 => tuples::main(),
        6 => arrays::main(),
        7 => vectors::main(),
        8 => conditionals::main(),
        9 => loops::main(),
        10 => functions::main(),
        11 => pointer_ref::main(),
        12 => structs::main(),
        13 => tuple_structs::main(),
        14 => basic_class::main(),
        15 => enums::main(),
        16 => cli::main(),
        _ => {
            println!(
                "Example {example_num} does not exist.",
                example_num = example_num
            )
        }
    }
}
