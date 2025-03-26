mod git;
mod toml;
use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;

fn run_phase(phase: toml::BuildPhase, dest: &str) {
    println!("Executing phase {}...", phase.name);
    let cmd_sequence = phase.commands.iter()
        .fold(String::new(), |cmd, next| -> String {
            String::from(cmd)+next+"; "
        });
    let _ = Command::new("/bin/bash")
        .current_dir(dest)
        .arg("-c")
        .arg(cmd_sequence)
        .status()
        .expect(&("Could not run phase".to_owned()+&phase.name));
}

fn main() {
    // parse config file
    println!("reading config file");
    let build_cfg = toml::read_build_cfg("build.toml").unwrap();

    // clone repo specified in config file
    println!("Cloning git repo");
    let dir = env::current_dir().unwrap();
    let stem = Path::new(&build_cfg.repo_name).file_stem().unwrap();
    let join = dir.join(stem);
    let dest = match join.to_str() {
        Some(str) => str,
        None => panic!("Could not write to file"),
    };
    git::clone(&build_cfg.repo_name, dest);

    // iterate over phases in config
    for phase in build_cfg {
        run_phase(phase, dest);
    }
    fs::remove_dir_all(dest).unwrap();
}
