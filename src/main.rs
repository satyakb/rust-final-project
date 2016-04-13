extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate time;

use docopt::Docopt;

pub mod swarm;
pub mod member;

use swarm::Swarm;

pub const VERSION: &'static str = "1.0";

const USAGE: &'static str = "
Swarm.

Usage:
  swarm [--num=<n>] HOST
  swarm (-h | --help)
  swarm (-v | --version)

Options:
  --num=<n>      Number of requests [default=10]
  -h, --help     Show this screen.
  -V, --version  Show version.
";

#[derive(Debug, RustcDecodable)]
/// Stores commandline arguments
struct Args {
    flag_num: i64,
    arg_HOST: Option<String>,
}

/// Parses commandline arguments and unleashes the swarm
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    let host = args.arg_HOST.unwrap();
    let num = args.flag_num;

    if num < 1 {
        println!("Error: please choose a number greater than 0");
        return
    }

    let mut swarm = Swarm::new(num, &host);
    println!("{:?}", swarm);
    println!("{:?}", swarm.unleash());
    println!("{:?}", swarm.stats());

}