pub fn handle_args(args: Vec<String>) -> (ProgramSelect, Vec<ProgramArgs>) {
    let mut program_select = ProgramSelect::None;
    let mut program_args = Vec::new();

    for arg in args {
        if matches!(program_select, ProgramSelect::None) {
            match arg.to_lowercase().as_str() {
                "--inspectdir" => program_select = ProgramSelect::Inspectdir,
                "--textimages" => program_select = ProgramSelect::TextImages,
                _ => continue,
            };
        } else {
            match arg.to_lowercase().as_str() {
                "-debug" => program_args.push(ProgramArgs::Debug),
                _ if arg.to_lowercase().starts_with("-path:") => {
                    program_args.push(ProgramArgs::Path(arg.replace("-path:", "")))
                }
                _ => continue,
            };
        }
    }

    (program_select, program_args)
}

#[derive(Debug)]
pub enum ProgramSelect {
    None,
    Inspectdir,
    TextImages,
}

#[derive(Debug)]
pub enum ProgramArgs {
    Debug,
    Path(String),
}

#[cfg(test)]
pub mod shared_tests {

    use super::*;

    #[test]
    fn program_select() {
        let test_args = vec![String::from("--inspectdir")];
        let (program, _args) = handle_args(test_args);
        assert!(matches!(program, ProgramSelect::Inspectdir));
    }

    #[test]
    fn program_args() {
        let test_args = vec![String::from("--inspectdir"), String::from("-debug")];
        let (_program, args) = handle_args(test_args);
        assert!(matches!(args[0], ProgramArgs::Debug));
    }

    #[test]
    fn program_args_path() {
        let test_args = vec![
            String::from("--inspectdir"),
            String::from("-path:/Users/benjaminpinter/Library/Caches"),
        ];
        let (_program, args) = handle_args(test_args);
        assert!(matches!(args[0], ProgramArgs::Path(_)));
    }
}
