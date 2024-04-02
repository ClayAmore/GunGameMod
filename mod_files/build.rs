// build.rs
use std::{env, fs};

// Copy dll to launcher folder to ease the dev workflow 
fn main() {
    let src_path = env::current_dir().unwrap()
        .join("target")
        .join(env::var("PROFILE").unwrap())
        .join("mod_files.dll");
    
    let dist_path = env::current_dir().unwrap()
        .parent().unwrap()
        .join("launcher")
        .join("mod_files.dll");
    
    if fs::copy(src_path, dist_path).is_err() {
        panic!("Failed to copy dll to launcher project.");
    };
}