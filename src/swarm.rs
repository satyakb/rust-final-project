use std::sync::{Arc, Mutex};
use std::thread;
use std::cmp::{min, max};
use member::Member;
use time::Duration;

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Swarm config
pub struct Config {
    /// Number of threads to generate/requests to send
    pub num: i64,

    /// Min request time in milliseconds
    pub host: String,
}

#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Contains all the calculated metrics
pub struct Stats {
    /// Mean request time in milliseconds
    pub mean: f64,

    /// Min request time in milliseconds
    pub min: i64,

    /// Max request time in milliseconds
    pub max: i64,

    /// Percentage of failed requests
    pub failed: f64,
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
    pub fn new(num: i64, host: &str) -> Swarm {
        Swarm {
            config: Config {
                num: num,
                host: host.to_string(),
            },
            members: Vec::with_capacity(num as usize),
        }
    }

    /// Generates all the necessary threads and sends the requests
    pub fn unleash(&mut self) -> Result<(), ()> {
        let members = Arc::new(Mutex::new(Vec::new()));
        let mut threads = Vec::new();

        for _ in 0..self.config.num {
            let members = members.clone();
            let host = self.config.host.clone();

            let thread = thread::spawn(move || {
                let mut members = members.lock().unwrap();
                let mut m = Member::new();
                m.send_request(&host);
                members.push(m);

            });
            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }

        self.members = Arc::try_unwrap(members).unwrap().into_inner().unwrap();
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
            mean: sum as f64 / self.config.num as f64,
            min: min_d.num_milliseconds(),
            max: max_d.num_milliseconds(),
            failed: 100.0 * num_fail as f64 / self.config.num as f64,
        }
    }
}