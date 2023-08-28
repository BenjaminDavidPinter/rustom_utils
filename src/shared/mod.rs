pub fn handle_args(args: Vec<String>) -> (ProgramSelect, Vec<ProgramArgs>) {
    let mut program_select = ProgramSelect::None;
    let mut program_args = Vec::new();

    for arg in args {
        if matches!(program_select, ProgramSelect::None) {
            match arg.to_lowercase().as_str() {
                "--cacheinspect" => program_select = ProgramSelect::CacheInspect,
                "--textimages" => program_select = ProgramSelect::TextImages,
                _ => continue
            };
        }
        else {
            match arg.to_lowercase().as_str() {
                "debug" => program_args.push(ProgramArgs::Debug),
                _ => continue,
            };
        }
    }

    return (program_select, program_args);
}

#[derive(Debug)]
pub enum ProgramSelect {
    None,
    CacheInspect,
    TextImages
}

#[derive(Debug)]
pub enum ProgramArgs{
    Debug
}

#[cfg(test)]
pub mod shared_tests {

    use super::*;

    #[test]
    fn program_select() {
        let test_args = vec![String::from("--CacheInspect")];
        let (program, _args) = handle_args(test_args);
        assert!(matches!(program, ProgramSelect::CacheInspect));
    }

    #[test]
    fn program_args(){
        let test_args = vec![String::from("--CacheInspect"), String::from("debug")];
        let (_program, args) = handle_args(test_args);
        assert!(matches!(args[0], ProgramArgs::Debug));
    }
}