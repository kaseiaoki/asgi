use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::string::String;
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
use std::path::PathBuf;

extern crate ice_t;

fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

fn lop(p: &std::path::PathBuf, target: &'static str) {
    let p = p.clone();
    thread::spawn(move || { pd(&p, target); });
    thread::sleep_ms(10);
}


fn check(d: Vec<String>, t: &'static str) {
    let mut catch: bool = false;
    let a: &'static str = &*t;
    for d in d.iter() {
        if (d.starts_with('.')) {
            continue;
        }
        let r = d.contains(&a);
        if (r == true) {
            let mut buf = PathBuf::from(d);
            let cd = env::current_dir().unwrap();
            println!("{} is in this directory -> {:?}", d, cd);
        }
    }
}


fn tos(p: &std::path::PathBuf, target: &'static str) {
    let names = ice_t::entry_to_string(&p);
    let t = String::from(target);
    check(names, target);
}

fn pd(p: &std::path::PathBuf, target: &'static str) {
    if let Ok(entries) = fs::read_dir(&p) {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                let md = metadata(entry.path()).unwrap();
                if (md.is_dir()) {
                    tos(&e, target);
                    let path_buf = entry.path();
                    lop(&path_buf, target);
                }
            }
        }
    }
}

fn sd(target: &'static str) {
    let t = target.clone();
    let cd = env::current_dir().unwrap();
    let file = ice_t::only_file_to_string(&cd);
    check(file, t);
    if let Ok(entries) = fs::read_dir("") {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                let md = metadata(entry.path()).unwrap();
                if (md.is_dir()) {
                    let path_buf = entry.path();
                    lop(&path_buf, t);
                }
            }
        }
    }
}

fn main() {
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword);
    let vec: Vec<&str> = keyword.split_whitespace().collect();
    let t = String::from(vec[0]);
    let s: &'static str = string_to_static_str(t);
    sd(s);
}
