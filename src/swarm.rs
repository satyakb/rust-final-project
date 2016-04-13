use std::sync::{Arc, Mutex};
use std::thread;
use member::Member;
use time::Duration;

#[derive(Debug)]
pub struct Swarm {
    count: i64,
    host: String,
    members: Vec<Duration>,
}

#[derive(Debug)]
pub struct Stats {
    mean: f64,
}

impl Swarm {
    pub fn new(count: i64, host: &str) -> Swarm {
        Swarm {
            count: count,
            host: host.to_string(),
            members: Vec::with_capacity(count as usize),
        }
    }

    pub fn unleash(&mut self) -> Result<(), ()> {
        let members = Arc::new(Mutex::new(Vec::new()));
        let mut threads = Vec::new();

        for _ in 0..self.count {
            let members = members.clone();
            let host = self.host.clone();
            let thread = thread::spawn(move || {
                let mut members = members.lock().unwrap();
                let m = Member::new();
                members.push(m.send_request(&host));

            });
            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }

        self.members = Arc::try_unwrap(members).unwrap().into_inner().unwrap();
        Ok(())
    }

    pub fn stats(&self) -> Stats {
        let mut sum = 0;
        for d in &self.members {
            sum += d.num_milliseconds();
        }
        Stats {
            mean: sum as f64 / self.count as f64,
        }
    }
}