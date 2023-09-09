use std::fs;


use super::shared::ProgramArgs;

//TODO: Make this relative somehow?
const CACHE_PATH: &str = "/Users/benjaminpinter/Library/Caches";

#[derive(Debug)]
struct FileDiveResult {
    file_name: String,
    total_size: u64,
    total_files: u64,
}

impl FileDiveResult {
    pub fn new(file: String) -> FileDiveResult {
        return FileDiveResult {
            total_size: 0,
            total_files: 0,
            file_name: file,
        };
    }
}

pub fn scan_cache(args: Vec<ProgramArgs>) {
    let mut scan_path = String::new();
    let mut debug_mode = false;
    for arg in args {
        match arg {
            ProgramArgs::Debug => debug_mode = true,
           ProgramArgs::Path(val) => scan_path = val,
        };
    }

    if debug_mode { println!("\r\n[SCAN PATH]: {}", scan_path) };

    let cache_dirs_result = fs::read_dir(&scan_path);
    let mut dirs = Vec::new();
    if let Ok(cache_dirs) = cache_dirs_result {
        for dir in cache_dirs {
            if let Ok(dir_info) = dir {
                if dir_info.path().as_path().is_dir() {
                    let this_path = dir_info.path();
                    dirs.push(dive_folder(
                        this_path.as_path().as_os_str().to_str().unwrap(),
                        1,
                    ));
                }
            }
        }
    }

    dirs.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    let mut max_dirs = dirs.len();

    if max_dirs > 5 {
        max_dirs = 5;
    }

    let top_5_offenders = &mut dirs[0..max_dirs];

    top_5_offenders.sort_by(|a, b| b.file_name.len().cmp(&a.file_name.len()));
    let longest_file_name = top_5_offenders.first().unwrap().file_name.len() + 3;

    top_5_offenders.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    println!("");
    println!("{:=^100}", scan_path);
    for offender in top_5_offenders {
        println!(
            "{:.<width$}{:.<25}{}",
            offender.file_name,
            format!("{} bytes", offender.total_size),
            format!("{} files", offender.total_files),
            width = longest_file_name
        );
    }
    println!("{:=^100}", "=");
    println!("");
}

fn dive_folder(path: &str, depth: i32) -> FileDiveResult {
    let mut file_dive_result = FileDiveResult::new(String::from(path));
    let cache_dirs_result = fs::read_dir(path);

    if let Ok(cache_dirs) = cache_dirs_result {
        for dir in cache_dirs {
            if let Ok(dir_info) = dir {
                if dir_info.path().as_path().is_dir() {
                    let this_path = dir_info.path();
                    let nested_result =
                        dive_folder(this_path.as_path().to_str().unwrap(), depth + 1);
                    file_dive_result.total_size += nested_result.total_size;
                    file_dive_result.total_files += nested_result.total_files;
                } else {
                    let metadata = fs::metadata(dir_info.path().as_path()).unwrap();
                    file_dive_result.total_size += metadata.len();
                    file_dive_result.total_files += 1;
                }
            }
        }
    }
    file_dive_result
}