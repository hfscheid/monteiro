use std::fs;
use std::io::{self, ErrorKind};
use homedir::my_home;
use crate::toml as buildcfg;

fn setup() -> io::Result<()> {
    // überprüfe, ob Verzeichnis schon existiert
    let home = my_home().unwrap();
    let monteiro_dir = &home.unwrap().join(".monteiro");
    match fs::exists(monteiro_dir) {
        Ok(true) => Ok(()),
        Ok(false) => fs::create_dir(monteiro_dir),
        _ => Err(io::Error::new(ErrorKind::PermissionDenied, "Could not create .monteiro directory")),
    }
}

fn add(filename: &str) -> io::Result<()> {
    if let Ok(_) = buildcfg::read_build_cfg(filename) {
        Ok(())
    } else {
        Err(io::Error::new(ErrorKind::InvalidInput, "Could not parse config file"))
    }
}

fn remove() {
}
