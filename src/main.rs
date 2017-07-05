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

fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}
fn check(d: Vec<String>, t: &'static str) -> bool {
    // println!("check");
    let mut catch: bool = false;
    let a: &'static str = &*t;
    for d in d.iter() {
        if (d.starts_with('.')) {
            continue;
        }
        let r = d.contains(&a);
        if (r == true) {
            println!("{} is in this directory", d);
            catch = true;
        }
    }
    catch
}

fn lop(p: &std::path::PathBuf, target: &'static str) {
    let p = p.clone();
    thread::spawn(move || { pd(&p, target); });
    thread::sleep_ms(10);
}


fn tos(p: &std::path::PathBuf, target: &'static str) -> bool {
    let paths = fs::read_dir(p).unwrap();
    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();

            let file_name = entry_path.file_name().unwrap();

            let file_name_as_str = file_name.to_str().unwrap();

            let file_name_as_string = String::from(file_name_as_str);

            file_name_as_string
        })
        .collect::<Vec<String>>();
    let dir: Vec<String> = names;
    if (dir.len() == 0) {
        return false;
    }
    let t = String::from(target);
    let ans = check(dir, target);
    ans
}

fn pd(p: &std::path::PathBuf, target: &'static str) {
    if let Ok(entries) = fs::read_dir(&p) {
        for entry in entries {
            if let Ok(entry) = entry {
                let e = entry.path();
                let md = metadata(entry.path()).unwrap();
                if (md.is_dir()) {
                    if (tos(&e, target)) {
                        println!("{:?}", entry.path());
                        // println!("{:?}", metadata(entry.path()).unwrap());
                    }
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
    tos(&cd, t);
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
