use toml::{self, de::Error};
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(std::fmt::Debug)]
#[derive(Clone)]
pub struct BuildPhase {
    pub name: String,
    pub commands: Vec<String>,
}

#[derive(Deserialize)]
#[derive(std::fmt::Debug)]
pub struct BuildCfg {
    #[serde(rename(deserialize = "repo-name"))]
    pub repo_name: String,
    phases: Vec<BuildPhase>,
    #[serde(skip)]
    i: usize
}

impl Iterator for BuildCfg {
    type Item = BuildPhase;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.phases.len() {
            self.i += 1;
            Some(self.phases[self.i-1].clone())
        } else {
            None
        }
    }
}

pub fn read_build_cfg(filename: &str) -> Result<BuildCfg, Error> {
    let mut cfg_file = File::open(filename).unwrap();
    let mut cfg_content = String::new();
    let _ = cfg_file.read_to_string(&mut cfg_content);
    toml::from_str::<BuildCfg>(&cfg_content)
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
            name = "build"
            commands = ["echo third", "echo fourth"]

        "#).unwrap();
        let expect = BuildCfg {
            repo_name: String::from("https://github.com/hfscheid/ktopology"),
            phases: [
                BuildPhase {
                    name: String::from("pre-build"),
                    commands: [
                        String::from("echo first"),
                        String::from("echo second"),
                    ].to_vec(),
                },
                BuildPhase {
                    name: String::from("build"),
                    commands: [
                        String::from("echo third"),
                        String::from("echo fourth"),
                    ].to_vec(),
                }
            ].to_vec(),
            i: 0
        };
        // println!("{:#?}", got);
        assert_eq!(got.repo_name, expect.repo_name);
        assert_eq!(got.phases[0].name, expect.phases[0].name);
        assert_eq!(got.phases[0].name, expect.phases[0].name);
        assert_eq!(got.phases[0].commands[0], expect.phases[0].commands[0]);
        assert_eq!(got.phases[0].commands[1], expect.phases[0].commands[1]);
        assert_eq!(got.phases[1].commands[0], expect.phases[1].commands[0]);
        assert_eq!(got.phases[1].commands[1], expect.phases[1].commands[1]);
    }
}
