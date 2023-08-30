use std::fs;
use std::path::Path;
use std::io;
use std::os::macos::fs::MetadataExt;

use super::shared::ProgramArgs;

const CACHE_PATH: &str = "/Users/benjaminpinter/Library/Caches";

/*NOTE: All I want to do is read the damn files on a disk, why is this family of fuctions designed
        like it's going to be used on lunar landers?


        First, we have fs::read_dir, which returns a Result<ReadDir>.
        When Ok(), Unwrap()-> ReadDir is an iter which exposes a 'list' of Result<DirEntry, Error>
        When Ok(), Unwrap() -> A DirEntry is finally what we want.

        ScanCache does this for a single file. The only difference between this iteration,
        and what will happen beneath, is one sends the first layer of folders to a recursive
        function, and the other...
        ...well it is the recursive function

*/
pub fn scan_cache(args : Vec<ProgramArgs>) {
    let cache_dirs_result = fs::read_dir(CACHE_PATH);
    match cache_dirs_result {
        Ok(cache_dirs) => {

            //cache_dirs is an iter() here?
            //NOTE: I breezed through the documentation for this Object, and there's
            //      really no interesting metadata on it. Also, what does the phrasing
            //      'Consumes the iterator' mean? Perhaps we should look into that...

            for dir in cache_dirs {
                //Let's unwrap this and then send the path to the recursive
                //  function
                match dir {
                    Ok(dir_info) => {
                        print!("{:?} - ", dir_info.file_name());

                        /*
                        It is finally at this point that we have reached the folders within
                        our cache.

                        I think, before we do this, we should check to see if the file
                        is a folder, or a file.
                        */


                        if dir_info.path().as_path().is_dir() {
                            let total_file_size = dive_folder(&dir_info.path().as_path(), 1);
                            println!("{} bytes", total_file_size);
                        }
                    },
                    _ => print!("Error")
                }
            }
        },
        _ => print!("Error")
    };
}

//So that means, in here we are going to do nearly the same thing, but
//  after printing out the directory tree, we call this again until we have no folders
//  left to traverse.
fn dive_folder(path: &Path, depth: i32) -> u64 {
    let mut total_file_size: u64 = 0;
    let prefix_string = generate_depth_tabs(depth);
    let cache_dirs_result = fs::read_dir(path.as_os_str());
    match cache_dirs_result {
        Ok(cache_dirs) => {
            for dir in cache_dirs {
                match dir {
                    Ok(dir_info) => {
                        if dir_info.path().as_path().is_dir() {
                            total_file_size += dive_folder(dir_info.path().as_path(), depth+1);
                        }
                        else {
                            let metadata = fs::metadata(dir_info.path().as_path()).unwrap();
                            total_file_size += metadata.st_size();
                        }
                    },
                    _ => total_file_size += 0
                }
            }
        },
        _ => total_file_size += 0
    };

    total_file_size
}

fn generate_depth_tabs(total: i32) -> String {
    let mut depth_string = String::new();
    for _ in 0..total {
        depth_string += "\t";
    }
    return depth_string;
}

#[cfg(test)]
pub mod cache_inspect_tests {
    use super::*;

    #[test]
    fn tab_depth_1() {
        let result = generate_depth_tabs(1);
        assert_eq!(result, "\t");
    }

    #[test]
    fn tab_depth_5() {
        let result = generate_depth_tabs(5);
        assert_eq!(result, "\t\t\t\t\t");
    }
}
