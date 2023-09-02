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
    let cache_dirs_result = fs::read_dir(CACHE_PATH);
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
    let top_5_offenders = &mut dirs[0..5];

    top_5_offenders.sort_by(|a, b| b.file_name.len().cmp(&a.file_name.len()));
    let longest_file_name = top_5_offenders.first().unwrap().file_name.len() + 3;

    top_5_offenders.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    println!("");
    println!("{:=^100}", "Top Cache Offenders");
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
