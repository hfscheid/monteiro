use crate::toml;
use crate::git;
use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;

pub fn run(build_cfg: toml::BuildCfg, clean: bool, status_stream: Option<()>) {
    // clone repo specified in config file
    println!("Cloning git repo");
    let dir = env::current_dir().unwrap();
    let stem = Path::new(&build_cfg.repo_name).file_stem().unwrap();
    let join = dir.join(stem);
    let project_dir = match join.to_str() {
        Some(str) => str,
        None => panic!("Could not write to file"),
    };
    git::clone(&build_cfg.repo_name, project_dir);

    // iterate over phases in config
    for phase in build_cfg {
        run_phase(phase, project_dir);
    }

    if clean {
        fs::remove_dir_all(project_dir).unwrap();
    }
}

pub fn run_phase(phase: toml::BuildPhase, project_dir: &str) {
    println!("Executing phase {}...", phase.name);
    let cmd_sequence = phase.commands.iter()
        .fold(String::new(), |cmd, next| -> String {
            String::from(cmd)+next+"; "
        });
    let _ = Command::new("/bin/bash")
        .current_dir(project_dir)
        .arg("-c")
        .arg(cmd_sequence)
        .status()
        .expect(&("Could not run phase".to_owned()+&phase.name));
}
