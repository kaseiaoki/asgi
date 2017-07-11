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


fn check(p: &std::path::PathBuf, t: &'static str) {
    let names = ice_t::entry_to_string(&p);
    let a: &'static str = &*t;
    for names in names.iter() {
        if names.contains(&a) {
            let name = &names;
            println!("{} -> {:?}", name, p);
        }
    }
}


fn pd(p: &std::path::PathBuf, target: &'static str) {
    if let Ok(entries) = fs::read_dir(&p) {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                if (metadata(entry.path()).unwrap().is_dir()) {
                    check(&e, target);
                    lop(&e, target);
                }
            }
        }
    }
}

fn sd(target: &'static str) {
    let t = target.clone();
    let cd = env::current_dir().unwrap();
    check(&cd, t);
    if let Ok(entries) = fs::read_dir("") {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                if (metadata(entry.path()).unwrap().is_dir()) {
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
