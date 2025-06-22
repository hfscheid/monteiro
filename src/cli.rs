use crate::arg;
use crate::toml;
use crate::run;

pub fn cli(args: &arg::Args) {
    let build_cfg: toml::BuildCfg;
    // parse config file
    println!("reading config file");
    match &args.montefile_path {
        Some(montefile_path) => {
            build_cfg = toml::read_build_cfg_from_path(&montefile_path)
                .unwrap();
        },
        None => {
            build_cfg = toml::read_build_cfg_from_stdin()
                .unwrap();
        }
    };
    run::run(build_cfg, args.clean, None);
}
