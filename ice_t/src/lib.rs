use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::string::String;
use std::iter::Iterator;
use std::io::prelude::*;
use std::os::windows;
// if you use unix os ↓
// use std::os::unix;
use std::path::Path;
use std::fs::metadata;
use std::env;
use std::vec::Vec;
use std::thread;
use std::process;
use std::mem;

pub fn entry_to_string(path: &std::path::PathBuf) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let is_String = paths
        .map(|entry| {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let file_name_as_str = file_name.to_str().unwrap();
            let file_name_as_string = String::from(file_name_as_str);
            file_name_as_string
        })
        .collect::<Vec<_>>();
    is_String
}

pub fn only_dir_to_string(path: &std::path::PathBuf) -> Vec<std::string::String> {
    let paths = fs::read_dir(path).unwrap();
    let is_dir = paths
        .map(|entry| {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let file_name_as_str = file_name.to_str().unwrap();
            let file_name_as_string = String::from(file_name_as_str);
            file_name_as_string
        })
        .filter(|path| metadata(&path).unwrap().is_dir() == true)
        .collect::<Vec<_>>();
    is_dir
}

pub fn only_file_to_string(path: &std::path::PathBuf) -> Vec<std::string::String> {
    let paths = fs::read_dir(path).unwrap();
    let is_file = paths
        .map(|entry| {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let file_name_as_str = file_name.to_str().unwrap();
            let file_name_as_string = String::from(file_name_as_str);
            file_name_as_string
        })
        .filter(|path| metadata(&path).unwrap().is_file() == true)
        .collect::<Vec<_>>();
    is_file
}

pub fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
pub fn current_dir_to_string() -> Vec<String> {
    let p = env::current_dir().unwrap();
    let names = entry_to_string(&p);
    names
}
