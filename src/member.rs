use time::{self, Duration};
use hyper::{self, Client};
use hyper::header::Connection;
use hyper::method::Method;
use hyper::status::StatusCode;

#[derive(Debug)]
/// Stores necessary parameters for sending a request
pub struct Member {
    /// Hyper client used to sent the request
    client: Client,

    /// Response StatusCode
    status_code: StatusCode,

    /// Whether or not request has been sent
    pub sent: bool,

    /// Whether or not request was successful
    pub success: bool,

    /// Request duration
    pub duration: Duration,
}

impl Member {

    /// Instantiates Member with a new client
    pub fn new() -> Member {
        Member {
            client: Client::new(),
            status_code: hyper::Ok,
            sent: false,
            success: false,
            duration: Duration::milliseconds(0),
        }
    }

    /// Sends the request, returns the time taken
    pub fn send_request(&mut self, host: &str) -> () {
        // Save starting time
        let start = time::get_time();

        let method = Method::Get;

        // Creating an outgoing request.
        let res = self.client.request(method, host)
            // set a header
            .header(Connection::close())
            // let 'er go!
            .send();

        // Save ending time
        let end = time::get_time();
        self.sent = true;

        match res {
            Ok(res) => {
                self.status_code = res.status;
                self.success = true;
                self.duration = end - start;
            },
            Err(_) => (),
        }
    }
}
