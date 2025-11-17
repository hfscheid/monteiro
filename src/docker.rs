use bollard;
use systemctl;
pub fn connect_daemon() -> Result<bollard::Docker, bollard::errors::Error> {
    bollard::Docker::connect_with_socket_defaults()
}
pub fn check_daemon() -> bool {
    let systemctl = systemctl::SystemCtl::default();
    match systemctl.status("docker") {
        Ok(status) => {
            status.contains(" active ")
        },
        Err(_) => false,
    }
}
