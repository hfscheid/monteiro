use std::fs;
use std::path::PathBuf;
use std::io::{self, ErrorKind, BufRead};
use homedir::my_home;
use serde_json;
use crate::toml as buildcfg;

fn home() -> PathBuf {
    let home = my_home().unwrap();
    home.unwrap().join(".monteiro")
}

fn read_index() -> serde_json::Value {
    let file_str = fs::read_to_string(home()).unwrap();
    serde_json::from_str(&file_str).unwrap()
}

fn setup() -> io::Result<()> {
    // überprüfe, ob Verzeichnis schon existiert
    // let home = my_home().unwrap();
    // let monteiro_dir = &home.unwrap().join(".monteiro");
    let monteiro_dir = &home();
    match fs::exists(monteiro_dir) {
        Ok(true) => Ok(()),
        Ok(false) => fs::create_dir(monteiro_dir),
        _ => Err(io::Error::new(ErrorKind::PermissionDenied, "Could not create .monteiro directory")),
    }
}

fn confirm_replace(bcfg: buildcfg::BuildCfg) -> io::Result<()> {
    print!("Build {} already exists. Replace? [y/n]: ", &bcfg.build_name);
    let mut opt = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut opt)
        .unwrap()
        .to_string();
    if opt == "y\n" {
        println!("Replacing...");
        write_cfg(bcfg)
    } else {
        println!("Preserving the original...");
        return Ok(())
    }
}

fn write_cfg(bcfg: buildcfg::BuildCfg) -> io::Result<()> {
    if let Some(path) = home().join(bcfg.build_name.to_owned()+".toml").to_str() {
        bcfg.to_file(path)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Could not write build configuration"))
    }
}

fn add(filename: &str) -> io::Result<()> {
    if let Ok(bcfg) = buildcfg::read_build_cfg(filename) {
        let index = read_index();
        return match index.get(&bcfg.build_name) {
            Some(_) => confirm_replace(bcfg),
            None => write_cfg(bcfg),
        };
    } else {
        Err(io::Error::new(ErrorKind::InvalidInput, "Could not parse config file"))
    }
}

fn remove() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_confirm_replace() {
        let _ = confirm_replace(buildcfg::BuildCfg::new("example"));
    }
}
