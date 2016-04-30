use std::thread;
use std::sync::mpsc::channel;
use std::cmp::{min, max};
use member::Member;
use time::Duration;

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Holds information needed to perform a request
pub struct Req {
    /// Path to make request to
    pub path: String,

    /// HTTP request method
    pub method: String,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Swarm config
pub struct Config {
    /// Number of threads to generate/requests to send
    pub num: i64,

    /// Min request time in milliseconds
    pub host: String,

    /// Sequence of requests to send
    pub seq: Vec<Req>,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Contains all the calculated metrics
pub struct Stats {
    /// Number of requests sent
    pub num: i64,

    /// Total time taken for all requests in milliseconds
    pub total: f64,

    /// Mean request time in milliseconds
    pub mean: f64,

    /// Min request time in milliseconds
    pub min: i64,

    /// Max request time in milliseconds
    pub max: i64,

    /// Percentage of failed requests
    pub failed: f64,
}

impl Stats {
    pub fn pretty_print(&self) {
        println!("{}:\t\t{}\r\n\
                {}:\t\t{}sec\r\n\
                {}:\t\t{}ms\r\n\
                {}:\t\t{}ms\r\n\
                {}:\t\t{}ms\r\n\
                {}:\t\t{}",
                "N", self.num,
                "Total", self.total,
                "Mean", self.mean,
                "Min", self.min,
                "Max", self.max,
                "%Fail", self.failed);
    }
}

impl Default for Stats {
    fn default() -> Stats {
        Stats {
            num: 0,
            total: 0.0,
            mean: 0.0,
            min: i64::max_value(),
            max: i64::min_value(),
            failed: 0.0,
        }
    }
}

#[derive(Debug)]
/// Keeps track of necessary parameters for the load testing
pub struct Swarm {
    /// Config
    config: Config,

    /// List of members
    members: Vec<Member>,
}

impl Swarm {
    /// Instantiates a new swarm with the given settings
    pub fn new(config: Config) -> Swarm {
        Swarm {
            config: config,
            members: Vec::new(),
        }
    }

    /// Generates all the necessary threads and sends the requests
    pub fn unleash(&mut self) -> Result<(), ()> {
        let mut threads = Vec::new();

        let (tx, rx) = channel();

        for _ in 0..self.config.num {
            let host = self.config.host.clone();
            let seq = self.config.seq.clone();

            let tx = tx.clone();

            let thread = thread::spawn(move || {
                let mut m = Member::new();
                m.send_request(&host, &seq);
                tx.send(m).unwrap();
            });
            threads.push(thread);
        }
        drop(tx);

        let mut members = Vec::new();
        while let Ok(m) = rx.recv() {
            members.push(m);
        }

        self.members = members;
        Ok(())
    }

    /// Returns the swarm stats
    pub fn stats(&self) -> Stats {
        let mut sum = 0;
        let mut num_fail = 0;
        let mut min_d = Duration::max_value();
        let mut max_d = Duration::min_value();
        for member in &self.members {
            let duration = member.duration;
            sum += duration.num_milliseconds();
            min_d = min(min_d, member.duration);
            max_d = max(max_d, member.duration);
            if !member.success {
                num_fail += 1;
            }
        }

        Stats {
            num: self.config.num,
            total: sum as f64 / 1000.0,
            mean: sum as f64 / self.config.num as f64,
            min: min_d.num_milliseconds(),
            max: max_d.num_milliseconds(),
            failed: 100.0 * num_fail as f64 / self.config.num as f64,
        }
    }
}