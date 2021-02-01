use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use walkdir::WalkDir;

const SRC_DIR: &str = "C:\\Users\\Ynit\\Desktop\\rust-os-pi";

const TARGET_DIR: &str = "C:\\Users\\Ynit\\Desktop\\rust-op-pi-git";

macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                continue;
            }
        }
    };
}
fn collect_project(dir_path: &mut Vec<(OsString, PathBuf)>) {
    // collect project from every folders
    for entry in WalkDir::new(SRC_DIR)
        .max_depth(1)
        .into_iter()
        .skip(1) {
        let entry = entry.unwrap();

        let _num = skip_fail! {entry.file_name().to_string_lossy()[0..2].parse::<u32>()};
        dir_path.push((entry.file_name().to_owned(), entry.into_path()));
    }
}


fn main() {
    git_init();
    let mut dir_path = vec![];

    collect_project(&mut dir_path);

    for (file_name, dir_path) in dir_path.iter() {
        clean_target_dir();
        copy_dir(&dir_path);
        git_add();
        git_commit(file_name.to_str().unwrap());
    }
}

fn copy_dir(source_dir: &dyn AsRef<Path>) {
    let buf = PathBuf::from(TARGET_DIR);

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .skip(1) {
        let entry = entry.unwrap();
        let file_type = entry.file_type();
        let src_path = entry.path();

        let file_path = src_path.strip_prefix(source_dir).unwrap();

        let mut target_path = buf.clone();
        target_path.push(file_path);

        if file_type.is_dir() {
            fs::create_dir(target_path).unwrap();
        } else {
            fs::copy(src_path, target_path).unwrap();
        }
    }
}


fn clean_target_dir() {
    for entry in WalkDir::new(TARGET_DIR)
        .max_depth(1)
        .into_iter()
        .skip(1) {
        let entry = entry.unwrap();

        let file_name = entry.file_name().to_string_lossy();
        if file_name == ".git" || file_name == ".gitignore" {
            continue;
        }
        if entry.file_type().is_dir() {
            fs::remove_dir_all(entry.path()).unwrap()
        } else {
            fs::remove_file(entry.path()).unwrap()
        }
    }
}


fn git_add() {
    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(TARGET_DIR)
        .output()
        .expect("failed to execute process");


    let status = output.status;
    if !status.success() {
        panic!("git add error")
    }
}

fn git_commit(file_name: &str) {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(file_name)
        .current_dir(TARGET_DIR)
        .output()
        .expect("failed to execute process");

    let status = output.status;
    if !status.success() {
        panic!("git commit error")
    }
}

fn git_init() {
    let output = Command::new("git")
        .arg("init")
        .current_dir(TARGET_DIR)
        .output()
        .expect("failed to execute process");

    let status = output.status;
    if !status.success() {
        panic!("git init error")
    }
}


