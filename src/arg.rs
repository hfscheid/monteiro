use clap::Parser;

// Parse CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'm', long = "montefile-path")]
    pub montefile_path: Option<String>,

    #[arg(short = 'c', long = "clean")]
    pub clean: bool,

    #[arg(short = 's', long = "server")]
    pub server: bool,

    #[arg(short = 'p', long = "port", default_value_t = 8080)]
    pub port: i32,
}
