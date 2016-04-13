use time::{self, Duration};
use hyper::Client;
use hyper::header::Connection;
use hyper::method::Method;

#[derive(Debug)]
pub struct Member {
    client: Client,
}

impl Member {
    pub fn new() -> Member {
        Member {
            client: Client::new(),
        }
    }

    pub fn send_request(&self, host: &str) -> Duration {
        // Save starting time
        let start = time::get_time();

        let method = Method::Get;

        // Creating an outgoing request.
        self.client.request(method, host)
            // set a header
            .header(Connection::close())
            // let 'er go!
            .send().unwrap();

        // Save ending time
        let end = time::get_time();
        end - start
    }
}
