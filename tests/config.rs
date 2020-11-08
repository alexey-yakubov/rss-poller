extern crate rss_poller;

use rss_poller::config::Config;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use serde_json;

#[test]
fn test_config_deserialization() {
    let lines = r#"
        [output]
            path ="/home/alexey/code/rss-poller/out"

        [input]
            links = [
            "http://feeds.reuters.com/news/artsculture",
            "http://feeds.reuters.com/reuters/businessNews",
            "http://feeds.reuters.com/reuters/companyNews"
            ]
    "#;
    let config = Config::load_from_str(lines).unwrap();

    assert_eq!("/home/alexey/code/rss-poller/out", config.output.path);
    assert_eq!("http://feeds.reuters.com/news/artsculture", config.input.links[0]);
    assert_eq!("http://feeds.reuters.com/reuters/businessNews", config.input.links[1]);
    assert_eq!("http://feeds.reuters.com/reuters/companyNews", config.input.links[2]);
}