use toml::Table;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

struct BuildPhase {
    name: String,
    commands: Vec<String>,
}

#[derive(Deserialize)]
pub struct BuildCfg {
    repo_name: String,
    phases: Vec<BuildPhase>
}

pub fn read_build_cfg(filename: &str) -> () {
    let mut cfg_file = File::open(filename).unwrap();
    let mut cfg_content = String::new();
    let _ = cfg_file.read_to_string(&mut cfg_content);
    let cfg = cfg_content.parse::<Table>().unwrap();

}
