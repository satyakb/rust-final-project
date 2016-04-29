use std::cmp::{min, max};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;
use yaml_rust::{Yaml, YamlLoader};

use swarm::{Config, Req, Stats};

/// Starts master, creates a thread per slave, returns aggregated stats
pub fn start(config_file: String) -> Stats {

    // Read Yaml File
    let path = Path::new(&config_file);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let docs = Arc::new(YamlLoader::load_from_str(&s).unwrap());

    // Set up threading stuff
    let mut threads = Vec::new();
    let mut stats = Arc::new(Mutex::new(Vec::new()));
    let count = docs[0]["slaves"].as_vec().unwrap().len();

    let host = docs[0]["host"].as_str().unwrap();
    let num = docs[0]["num"].as_i64().unwrap();

    let seq = docs[0]["sequence"].as_vec().unwrap();
    let mut seq_parsed = Vec::new();
    for s in seq {
        let s = s.as_hash().unwrap();
        let r = Req {
            path: s.get(&Yaml::from_str("path")).unwrap().as_str().unwrap().to_string(),
            method: s.get(&Yaml::from_str("method")).unwrap().as_str().unwrap().to_string(),
        };
        seq_parsed.push(r);
    }

    // Parse config file
    let body = Config {
        num: num,
        host: host.to_string(),
        seq: seq_parsed,
    };
    let body = json::encode(&body).unwrap();

    // Create threads
    for i in 0..count {
        let mut stats = stats.clone();
        let body = body.clone();
        let docs = docs.clone();

        let thread = thread::spawn(move || {
            let doc = &docs[0];
            let slave_ip = &doc["slaves"][i].as_str().unwrap();
            let url = "http://".to_string() + slave_ip;

            let stat = trigger_slave(&url, &body);

            let mut stats = stats.lock().unwrap();
            stats.push(stat);
        });

        threads.push(thread);
    }

    // Wait for threads to finish
    for thread in threads {
        match thread.join() {
            Ok(_) => (),
            Err(_) => {
                println!("Error: unreachable slave");
            },
        }
    }

    let stats = Arc::try_unwrap(stats).unwrap().into_inner().unwrap();
    aggregate_stats(stats)
}

/// Triggers slave to unleash its swarm
fn trigger_slave(url: &str, body: &str) -> Stats {
    // Create client
    let client = Client::new();

    // Creates an outgoing request.
    let mut res = client.post(url)
        .body(body)
        .header(Connection::close())
        .send().unwrap();

    let mut buf = String::new();
    res.read_to_string(&mut buf).unwrap();

    // Decode stats
    let stat : Stats = json::decode(&buf).unwrap();
    stat
}

/// Aggregates a vector of stats
fn aggregate_stats(stats: Vec<Stats>) -> Stats {
    let mut ret_stat : Stats = Default::default();

    for stat in &stats {
        ret_stat.num += stat.num;
        ret_stat.total += stat.total;
        ret_stat.mean += stat.mean;
        ret_stat.min = min(ret_stat.min, stat.min);
        ret_stat.max = max(ret_stat.max, stat.max);
        ret_stat.failed += stat.failed;
    }

    if stats.len() > 0 {
        ret_stat.mean /= stats.len() as f64;
        ret_stat.failed /= stats.len() as f64;
    }

    ret_stat
}