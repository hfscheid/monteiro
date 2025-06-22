use crate::toml;
use crate::git;
use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
use std::fs;
use std::sync::mpsc::Sender;
use std::io::{BufReader, BufRead};

pub fn run(build_cfg: toml::BuildCfg,
           clean: bool,
           mut status_stream: Option<Sender<String>>) {
    // clone repo specified in config file
    match status_stream {
        Some(ref s_stream) => {
            if let Err(e) = s_stream.send(String::from("Cloning git repo")) {
                panic!("Error: {}", e);
            }
        },
        None => println!("Cloning git repo"),
    };
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
        run_phase(phase, project_dir, &mut status_stream);
    }

    if clean {
        fs::remove_dir_all(project_dir).unwrap();
    }
}

pub fn run_phase(phase: toml::BuildPhase,
                 project_dir: &str,
                 status_stream: &mut Option<Sender<String>>) {
    match status_stream {
        Some(s_stream) => {
            if let Err(e) = s_stream.send(
                format!("Executing phase {}...", phase.name)
            ) {
                panic!("Error: {}", e);
            }
        },
        None => println!("Executing phase {}...", phase.name),
    };
    let cmd_sequence = phase.commands.iter()
        .fold(String::new(), |cmd, next| -> String {
            String::from(cmd)+next+"; "
        });
    let child_process = Command::new("/bin/bash")
        .current_dir(project_dir)
        .arg("-c")
        .arg(cmd_sequence)
        .stdout(Stdio::piped())
        .spawn()
        .expect(&("Could not run phase".to_owned()+&phase.name));
        if let Some(s_stream) = status_stream {
            let output = child_process.stdout.unwrap();
            let mut buf_reader = BufReader::new(output);
            let mut line = String::new();
            loop {
                buf_reader.read_line(&mut line);
                if line.is_empty() {
                    break;
                }
                if let Err(e) = s_stream.send(
                    line.clone().trim().to_string()
                ) {
                    panic!("Error: {}", e);
                }
                line.clear();
            };
        };
}
