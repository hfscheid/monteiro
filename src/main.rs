mod git;
use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;

fn main() {
    let repo_name = "https://github.com/hfscheid/ktopology";
    let dir = env::current_dir().unwrap();
    let stem = Path::new(repo_name).file_stem().unwrap();
    let join = dir.join(stem);
    let dest = match join.to_str() {
        Some(str) => str,
        None => panic!("Could not write to file"),
    };
    git::clone("https://github.com/hfscheid/ktopology", dest);

    let _ = Command::new("/bin/bash")
        .current_dir(dest)
        .arg("-c")
        .arg("echo hop; echo ha")
        .status()
        .expect("Could not list directory");
    fs::remove_dir_all(dest).unwrap();
}
