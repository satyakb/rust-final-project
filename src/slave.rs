use std::io::Read;

use hyper::{self, Server};
use hyper::server::Response;
use hyper::server::request::Request;
use hyper::status::StatusCode;
use rustc_serialize::json;

use swarm::{Config, Stats, Swarm};

/// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr, $res:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            *($res).status_mut() = StatusCode::InternalServerError;
            return;
        }
    })
}

/// Handles request from master and unleashes swarm
fn req_handler(mut req: Request, mut res: Response) {
    match req.method {
        hyper::Post => {

            // Read request
            let mut buf = String::new();
            try_or_server_err!(req.read_to_string(&mut buf), res);

            // Make sure request is a Config
            let config : Config = try_or_server_err!(json::decode(&buf), res);

            // Unleash the swarm
            let stats = unleash(config);
            let stats_string = try_or_server_err!(json::encode(&stats), res);

            // Send back OK and the stats
            *res.status_mut() = StatusCode::Ok;
            res.send(&stats_string.as_bytes()).unwrap();
        },
        _ => *res.status_mut() = StatusCode::NotFound,
    }
}

/// Starts http server listening for requests from the master
pub fn start(ip: &str, port: i64) {
    let addr = format!("{}:{}", ip, port);
    println!("Please add {} to the config.yaml on the master node.", addr);
    match Server::http(addr.as_str()) {
        Ok(server) => {
            match server.handle(req_handler) {
                Ok(_) => (),
                Err(e) => println!("{:?}", e),
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

/// Unleashes the swarm from local machine
pub fn unleash(config: Config) -> Stats {
    let mut swarm = Swarm::new(config);
    println!("{:?}", swarm);
    swarm.unleash().unwrap();
    println!("{:?}", swarm.stats());

    swarm.stats()
}