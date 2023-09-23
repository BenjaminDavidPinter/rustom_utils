use super::shared::ProgramArgs;
use std::{fs, env, path::PathBuf, path::Path};

#[derive(Debug)]
struct FileDiveResult {
    file_name: String,
    total_size: u64,
    total_files: u64,
}

impl FileDiveResult {
    pub fn new(file: String) -> FileDiveResult {
        FileDiveResult {
            total_size: 0,
            total_files: 0,
            file_name: file,
        }
    }
}

pub fn inspect(args: Vec<ProgramArgs>) {
    let mut scan_path = String::new();
    let mut debug_mode = false;
    for arg in args {
        match arg {
            ProgramArgs::Debug => debug_mode = true,
            ProgramArgs::Path(val) => {
                let mut dir_buf = PathBuf::new();
                dir_buf.push(val);
                scan_path = get_path(dir_buf);
            }
        };
    }

    debug_print(debug_mode, format!("{:<20}{}", "[scan_path]:", scan_path));

    let cache_dirs_result = fs::read_dir(&scan_path);
    let mut dirs = Vec::new();
    if let Ok(cache_dirs) = cache_dirs_result {
        for dir_info in cache_dirs.into_iter().flatten() {
            if dir_info.path().as_path().is_dir() {
                let this_path = dir_info.path();
                dirs.push(dive_folder(this_path));
            }
        }
    }

    dirs.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    let mut max_dirs = dirs.len();
    debug_print(debug_mode, format!("{:<20}{}", "[max_dirs]:", max_dirs));

    if max_dirs > 5 {
        max_dirs = 5;
    }

    let top_5_offenders = &mut dirs[0..max_dirs];

    top_5_offenders.sort_by(|a, b| b.file_name.len().cmp(&a.file_name.len()));
    let longest_file_name = top_5_offenders.first().unwrap().file_name.len() + 3;

    top_5_offenders.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    let nested_folder_pad = scan_path.len()+1;

    println!();
    println!("{}/", scan_path);
    for offender in top_5_offenders {
        println!(
            "{: <nested_folder_pad$}{:.<longest_file_name$}{:.<10}{}",
            "",
            offender.file_name,
            format!("{}", to_friendly_byte_name(offender.total_size)),
            format!("{} files", offender.total_files)
        );
    }
    println!();
}

fn dive_folder(path: PathBuf) -> FileDiveResult {
    let mut file_dive_result = FileDiveResult::new(String::from(path.as_path().file_stem().unwrap().to_str().unwrap()));
    let cache_dirs_result = fs::read_dir(path);

    if let Ok(cache_dirs) = cache_dirs_result {
        for dir_info in cache_dirs.into_iter().flatten() {
            if dir_info.path().as_path().is_dir() {
                let this_path = dir_info.path();
                let nested_result =
                    dive_folder(this_path);
                file_dive_result.total_size += nested_result.total_size;
                file_dive_result.total_files += nested_result.total_files;
            } else {
                let metadata = fs::metadata(dir_info.path().as_path()).unwrap();
                file_dive_result.total_size += metadata.len();
                file_dive_result.total_files += 1;
            }
        }
    }
    file_dive_result
}

fn debug_print(debug_mode: bool, message: String) {
    if debug_mode {
        println!("{}",message);
    }
}

fn to_friendly_byte_name(total_bytes: u64) -> String {
    let mut iterations = 0;
    let mut final_value = total_bytes;

    while final_value > 1024 {
        final_value /= 1024;
        iterations += 1;
    }

    match iterations {
        0 => format!("{}b", final_value),
        1 => format!("{}kb", final_value, ),
        2 => format!("{}mb", final_value, ),
        3 => format!("{}gb", final_value, ),
        4 => format!("{}tb", final_value),
        _ => panic!("Too large to calculate")
    }
}


fn get_path(dir: PathBuf) -> String {
    if dir.is_absolute() {
        return String::from(dir.to_str().unwrap());
    } else {
        let pwd = env::current_dir().unwrap().join(dir);
        return String::from(pwd.to_str().unwrap());
    }
}

#[cfg(test)]
pub mod inspectdir_tests {
    use super::*;

}
