extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate time;
extern crate yaml_rust;

use docopt::Docopt;

pub mod constants;
pub mod member;
pub mod master;
pub mod swarm;
mod slave;

use constants::{USAGE, VERSION};
use swarm::{Config, Stats};

#[derive(Debug, RustcDecodable)]
/// Stores commandline arguments
struct Args {
    flag_port: i64,
    flag_num: i64,
    arg_cfg: Option<String>,
    arg_host: Option<String>,
    cmd_unleash: bool,
    cmd_master: bool,
    cmd_slave: bool,
}

/// Parses commandline arguments
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());

    if args.cmd_slave {
        slave::start(args.flag_port);
    } else {

        let mut stat : Stats = Default::default();

        if args.cmd_master {
            let config_file = args.arg_cfg.unwrap();
            stat = master::start(config_file);

        } else {

            let host = args.arg_host.unwrap();
            let num = args.flag_num;

            if num < 1 {
                println!("Error: please choose a number greater than 0");
                return
            }

            let config = Config {
                num: num,
                host: host,
            };
            stat = slave::unleash(config);

        }

        println!("{:?}", stat);
    }
}