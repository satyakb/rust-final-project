use std::cmp::{min, max};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;
use yaml_rust::{YamlLoader, Yaml};

use swarm::{Config, Stats};

/// Starts master
/// Creates a thread per slave and sends command to the slave to unleash swarm
/// Returns aggregated stats
pub fn start(config: Config) -> Stats {
    let body = json::encode(&config).unwrap();

    // Read Yaml File
    let mut file = File::open("/Users/satyakb/School/cis198/swarm/src/config.yaml").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let docs = Arc::new(YamlLoader::load_from_str(&s).unwrap());

    // Set up threading stuff
    let mut threads = Vec::new();
    let mut stats = Arc::new(Mutex::new(Vec::new()));
    let count = docs[0]["slaves"].as_vec().unwrap().len();

    for i in 0..count {
        // Creating an outgoing request.
        let mut stats = stats.clone();
        let body = body.clone();
        let docs = docs.clone();

        let thread = thread::spawn(move || {
            let doc = &docs[0];
            let slave_ip = &doc["slaves"][i].as_str().unwrap();
            println!("{:?}", slave_ip);
            let url = "http://".to_string() + slave_ip;
            let client = Client::new();
            let mut res = client.post(&url)
                .body(&body)
                .header(Connection::close())
                .send().unwrap();

            let mut buf = String::new();
            res.read_to_string(&mut buf).unwrap();

            let stat : Stats = json::decode(&buf).unwrap();
            let mut stats = stats.lock().unwrap();
            stats.push(stat);
        });

        threads.push(thread);
    }

    // Wait for threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    // Aggregate the Stats
    let mut ret_stat = Stats {
        mean: 0.0,
        min: i64::max_value(),
        max: i64::min_value(),
        failed: 0.0,
    };
    let stats = Arc::try_unwrap(stats).unwrap().into_inner().unwrap();
    for stat in &stats {
        ret_stat.mean += stat.mean;
        ret_stat.min = min(ret_stat.min, stat.min);
        ret_stat.max = max(ret_stat.max, stat.max);
        ret_stat.failed += stat.failed;
    }
    ret_stat.mean /= stats.len() as f64;
    ret_stat.failed /= stats.len() as f64;

    ret_stat
}