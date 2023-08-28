use std::env;
pub mod shared;

fn main() {
    let (program_select, program_args) = shared::handle_args(env::args());
    println!("{:?}", program_select);
    for arg in program_args {
        println!("{:?}", arg);
    }
}