use crate::docker;
use std::fmt;

pub enum StartupError {
    DockerConnectError,
}
// #[derive(Debug, Clone)]
// struct StartupError;
impl fmt::Display for StartupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &StartupError::DockerConnectError => {
                write!(f, "Could not connect do Docker daemon.")
            },
        }
    }
}

pub fn check() -> Result<(), StartupError> {
    if !docker::check_daemon() {
        return Err(StartupError::DockerConnectError);
    }
    Ok(())
}
