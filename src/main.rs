use std::env;
pub mod shared;
use shared::*;
pub mod inspectdir;

fn main() {
    let (program_select, program_args) = shared::handle_args(env::args().collect());

    match program_select {
        ProgramSelect::Inspectdir => inspectdir::inspect(program_args),
        _ => panic!("Unsupported program")
    };
}