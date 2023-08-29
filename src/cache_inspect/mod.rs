use std::fs;
use std::path::Path;

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
pub fn scan_cache() {
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
                        println!("{:?}", dir_info);

                        /*
                        It is finally at this point that we have reached the folders within
                        our cache.

                        I think, before we do this, we should check to see if the file
                        is a folder, or a file.
                        */


                        if dir_info.path().as_path().is_dir() {
                            dive_folder(&dir_info.path().as_path(), 1);
                        }
                        else {
                            //This isolates files which are top-level non-directory files.
                            // This inludes raw
                            println!("F:{:?}", dir_info);
                        }


                    },
                    _ => println!("An error has occured")
                }
            }
        },
        _ => println!("An error occured")
    };
}

//So that means, in here we are going to do nearly the same thing, but
//  after printing out the directory tree, we call this again until we have no folders
//  left to traverse.
fn dive_folder(path: &Path, depth: i32) {
    let prefix_string = generate_depth_tabs(depth);
    let cache_dirs_result = fs::read_dir(path.as_os_str());
    match cache_dirs_result {
        Ok(cache_dirs) => {
            for dir in cache_dirs {
                match dir {
                    Ok(dir_info) => {
                        println!("{}{:?}", prefix_string, dir_info);
                        if dir_info.path().as_path().is_dir() {
                            dive_folder(dir_info.path().as_path(), depth+1);
                        }
                        else {
                            println!("{}F:{:?}", prefix_string,dir_info);
                        }
                    },
                    _ => println!("An error has occured")
                }
            }
        },
        _ => println!("An error occured")
    };
}

fn generate_depth_tabs(total: i32) -> String{
    //Move this into its own function, it's messy in here
    let mut depth_string = String::new();
    for i in 0..total {
        depth_string += "\t";
    }
    return depth_string;
}
