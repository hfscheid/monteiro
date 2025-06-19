use toml::{self, de::Error};
use std::fs::{self, File};
use std::io::{self, Read};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(std::fmt::Debug)]
#[derive(Clone)]
pub struct BuildPhase {
    pub name: String,
    pub commands: Vec<String>,
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(std::fmt::Debug)]
pub struct BuildCfg {
    #[serde(rename(deserialize = "build-name"))]
    pub build_name: String,
    #[serde(rename(deserialize = "remote-url"))]
    pub repo_name: String,
    phases: Vec<BuildPhase>,
    #[serde(skip)]
    i: usize
}

impl BuildCfg {
    pub fn new(name: &str) -> BuildCfg {
        return BuildCfg {
            build_name: String::from(name),
            repo_name: String::from(""),
            phases: Vec::<BuildPhase>::new(),
            i: 0
        }
    }
    pub fn to_file(&self, filename: &str) -> io::Result<()> {
        let data = toml::to_string(self).unwrap();
        fs::write(filename, data)
    }
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
            build-name = "test build"
            repo-name = "https://github.com/hfscheid/ktopology"
            [[phases]]
            name = "pre-build"
            commands = ["echo first", "echo second"]

            [[phases]]
            name = "build"
            commands = ["echo third", "echo fourth"]

        "#).unwrap();
        let expect = BuildCfg {
            build_name: String::from("test build"),
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
        assert_eq!(got.build_name, expect.build_name);
        assert_eq!(got.repo_name, expect.repo_name);
        assert_eq!(got.phases[0].name, expect.phases[0].name);
        assert_eq!(got.phases[0].name, expect.phases[0].name);
        assert_eq!(got.phases[0].commands[0], expect.phases[0].commands[0]);
        assert_eq!(got.phases[0].commands[1], expect.phases[0].commands[1]);
        assert_eq!(got.phases[1].commands[0], expect.phases[1].commands[0]);
        assert_eq!(got.phases[1].commands[1], expect.phases[1].commands[1]);
    }
}
