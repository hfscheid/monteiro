mod git;
mod toml;
mod cli;
mod arg;
mod run;
mod server;
mod startup;
mod docker;
// mod data;

use clap::Parser;

fn main() {
    if let Err(e) = startup::check() {
        println!("[main] Startup fail: {}", e);
        return
    }
    let args = arg::Args::parse();
    if args.server {
    // run in server mode
        server::serve(args.port)
    } else {
    // run as cli
        cli::cli(&args)
    }
}
