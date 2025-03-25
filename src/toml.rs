use toml::Table;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(std::fmt::Debug)]
struct BuildPhase {
    name: String,
    commands: Vec<String>,
}

#[derive(Deserialize)]
#[derive(std::fmt::Debug)]
pub struct BuildCfg {
    #[serde(rename(deserialize = "repo-name"))]
    repo_name: String,
    phases: Vec<BuildPhase>
}

pub fn read_build_cfg(filename: &str) -> () {
    let mut cfg_file = File::open(filename).unwrap();
    let mut cfg_content = String::new();
    let _ = cfg_file.read_to_string(&mut cfg_content);
    let cfg = cfg_content.parse::<Table>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_build() {
        let got: BuildCfg = toml::from_str(r#"
            repo-name = "https://github.com/hfscheid/ktopology"
            [[phases]]
            name = "pre-build"
            commands = ["echo first", "echo second"]

            [[phases]]
            name = "pre-build"
            commands = ["echo third", "echo fourth"]

        "#).unwrap();
        println!("{:#?}", got);
    }
}
