extern crate observer;

use observer::HttpClient;
use observer::Logger;

fn main() {
    let mut  http_stream = HttpClient::new("httpbin.org", 80);
    http_stream.add_events_hook(Logger);
    http_stream.get("/ip");
    // http_stream.get("/uuid");
}