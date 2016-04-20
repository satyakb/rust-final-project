use std::io::Read;
use hyper::{self, Server};
use hyper::server::Response;
use hyper::server::request::Request;
use hyper::status::StatusCode;
use rustc_serialize::json;

use constants::SLAVE_ADDR;
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

fn req_handler(mut req: Request, mut res: Response) {
    match req.method {
        hyper::Post => {

            // Read request
            let mut buf = String::new();
            try_or_server_err!(req.read_to_string(&mut buf), res);

            // Make sure request is a Config
            let config : Config = try_or_server_err!(json::decode(&buf), res);
            println!("{:?}", config);

            let stats = unleash(config);
            let stats_string = try_or_server_err!(json::encode(&stats), res);

            *res.status_mut() = StatusCode::Ok;
            res.send(&stats_string.as_bytes()).unwrap();
        },
        _ => *res.status_mut() = StatusCode::NotFound,
    }
}

pub fn start() {
    println!("Listening on {}.", SLAVE_ADDR);
    match Server::http(SLAVE_ADDR) {
        Ok(server) => match server.handle(req_handler) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        },
        Err(e) => println!("{:?}", e),
    }
}

/// Unleashes the swarm from local computer
pub fn unleash(config: Config) -> Stats {
    let mut swarm = Swarm::new(config.num, &config.host);
    println!("{:?}", swarm);
    println!("{:?}", swarm.unleash());
    println!("{:?}", swarm.stats());

    swarm.stats()
}