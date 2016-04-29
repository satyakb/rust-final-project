extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate time;
extern crate yaml_rust;
extern crate libc;
extern crate core;

extern {
    fn get_ip_of_interface(iface: *const libc::c_char) -> *const libc::c_char;
}

use std::ffi::{CStr, CString};
use std::str;
use docopt::Docopt;

pub mod constants;
pub mod member;
pub mod master;
pub mod swarm;
mod slave;

use constants::{USAGE, VERSION};
use swarm::Config;

#[derive(Debug, RustcDecodable)]
/// Stores commandline arguments
struct Args {
    flag_num: i64,
    flag_port: i64,
    flag_iface: Option<String>,
    arg_cfg: Option<String>,
    arg_host: Option<String>,
    cmd_unleash: bool,
    cmd_master: bool,
    cmd_slave: bool,
}

/// Naively gets default internet interface based on OS
fn get_default_iface() -> String {
    if std::env::consts::OS == "macos" {
        return "en0".to_string();
    }

    return "eth0".to_string();
}

/// Parses commandline arguments
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());

    // Get IP address
    let iface = args.flag_iface.unwrap_or_else(get_default_iface);

    let iface = CString::new(iface.as_str()).unwrap();
    let c_buf: *const libc::c_char = unsafe { get_ip_of_interface(iface.as_ptr()) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buf: &[u8] = c_str.to_bytes();
    let ip: &str = str::from_utf8(buf).unwrap();

    if args.cmd_slave {
        slave::start(ip, args.flag_port);

    } else {

        let stat;

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
                seq: Vec::new(),
            };
            stat = slave::unleash(config);

        }

        stat.pretty_print();
    }
}