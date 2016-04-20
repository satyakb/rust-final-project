extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate time;

use docopt::Docopt;

pub mod constants;
pub mod member;
pub mod master;
pub mod swarm;
mod slave;

use constants::{USAGE, VERSION};
use swarm::{Config};

#[derive(Debug, RustcDecodable)]
/// Stores commandline arguments
struct Args {
    flag_num: i64,
    arg_HOST: Option<String>,
    cmd_unleash: bool,
    cmd_master: bool,
    cmd_slave: bool,
}

/// Parses commandline arguments
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if args.cmd_slave {
        slave::start();
    } else {
        let host = args.arg_HOST.unwrap();
        let num = args.flag_num;

        if num < 1 {
            println!("Error: please choose a number greater than 0");
            return
        }

        if args.cmd_master {
            unimplemented!();
        } else {
            slave::unleash(Config {num: num, host: host});
        }
    }
}