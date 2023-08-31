use std::fs;

use super::shared::ProgramArgs;

//TODO: Make this relative somehow?
const CACHE_PATH: &str = "/Users/benjaminpinter/Library/Caches";

/*
okay okay okay;
&str <- const string data
String <- growable heap string data

file_name <- String because we must assign it at runtime, we don't know the length
const strings are &str meaning 'borrowed' string data from somewhere else. 'Else' here being ROM.

So I'm making the following assumption;
file names are stored in OS, ROM memory, which is why I should be able to access the value as a &str.
But I want to make a copy of that memory for my use, so I need to borrow that.
*/
#[derive(Debug)]
struct FileDiveResult {
    //I really don't want to make this a String, but since now, it's borrowed, I need to make sure it lives as long as the owning struct.
    //  but it's so obvious that it will, because I'm only going to source this from the operating system.

    //I can't tell if this is one of those times where my laziness wins, or I'm being pragmatic;
    //Getting this to work with file_name being a &str is very 'jumping though hoops' for my level of skill with rust right now.
    //So I'm just going to make a copy of the data.
    file_name: String,
    total_size: u64,
    total_files: u64
}

//Because our struct now has a lifetime, we must also specify that lifetime during the impl block
impl FileDiveResult{
    pub fn new(file: String) -> FileDiveResult {
        return FileDiveResult { total_size: 0, total_files: 0, file_name: file }
    }
}

/*NOTE: All I want to do is read the damn files on a disk, why is this family of fuctions designed
        like it's going to be used on lunar landers?


        First, we have fs::read_dir, which returns a Result<ReadDir>.
        When Ok(), Unwrap()-> ReadDir is an iter which exposes a 'list' of Result<DirEntry, Error>
        When Ok(), Unwrap() -> A DirEntry is finally what we want.

        ScanCache does this for a single file. The only difference between this iteration,
        and what will happen beneath, is one sends the first layer of folders to a recursive
        function, and the other...
        ...well it is the recursive function

        There's probably a much more elegant way to handle the shared code.
Í›44
*/
pub fn scan_cache(args : Vec<ProgramArgs>) {
    let cache_dirs_result = fs::read_dir(CACHE_PATH);
    let mut dirs = Vec::new();
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

                        /*
                        It is finally at this point that we have reached the folders within
                        our cache.

                        I think, before we do this, we should check to see if the file
                        is a folder, or a file.
                        */
                        if dir_info.path().as_path().is_dir() {
                            let this_path = dir_info.path();
                            dirs.push(dive_folder(this_path.as_path().as_os_str().to_str().unwrap(), 1));
                        }
                    },
                    _ => print!("Error")
                }
            }
        },
        _ => print!("Error")
    };


    dirs.sort_by(|a,b| b.total_size.cmp(&a.total_size));
    let top_5_offenders = &dirs[0..5];

    for offender in top_5_offenders {
        println!("{:.<70}{:.<25}{} files",

        offender.file_name,
        format!("{} bytes", offender.total_size),
        offender.total_files);
    }
}

//So that means, in here we are going to do nearly the same thing, but
//  after printing out the directory tree, we call this again until we have no folders
//  left to traverse.
fn dive_folder(path: &str, depth: i32) -> FileDiveResult {
    let mut file_dive_result = FileDiveResult::new(String::from(path));

    let prefix_string = generate_depth_tabs(depth);
    let cache_dirs_result = fs::read_dir(path);
    match cache_dirs_result {
        Ok(cache_dirs) => {
            for dir in cache_dirs {
                match dir {
                    Ok(dir_info) => {
                        if dir_info.path().as_path().is_dir() {
                            let this_path = dir_info.path();
                            let nested_result = dive_folder(this_path.as_path().to_str().unwrap(), depth+1);
                            file_dive_result.total_size += nested_result.total_size;
                            file_dive_result.total_files += nested_result.total_files;
                        }
                        else {
                            let metadata = fs::metadata(dir_info.path().as_path()).unwrap();
                            file_dive_result.total_size += metadata.len();
                            file_dive_result.total_files += 1;
                        }
                    },
                    _ => file_dive_result.total_size += 0
                }
            }
        },
        _ => file_dive_result.total_size += 0
    };

    file_dive_result
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
