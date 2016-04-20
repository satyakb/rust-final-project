#[derive(Debug, Clone)]
#[derive(RustcDecodable, RustcEncodable)]
/// Contains all the calculated metrics
pub struct Config {
    /// Number of threads to generate/requests to send
    pub num: i64,

    /// Min request time in milliseconds
    pub host: String,
}