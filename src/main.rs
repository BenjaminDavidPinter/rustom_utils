use std::env;
pub mod shared;
use shared::*;
pub mod cache_inspect;
use cache_inspect::*;

fn main() {
    let (program_select, program_args) = shared::handle_args(env::args().collect());

    match program_select {
        ProgramSelect::CacheInspect => cache_inspect::scan_cache(),
        _ => panic!("Unsupported program")
    };
}