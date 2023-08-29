use std::fs;
use std::path::Path;

const CACHE_PATH: &str = "/Users/benjaminpinter/Library/Caches";

pub fn scan_cache() {
    let cache_dirs = fs::read_dir(CACHE_PATH);
}

fn dive_folder(path: &Path) {
    
}
