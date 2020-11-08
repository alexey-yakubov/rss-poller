use serde::{Serialize, Deserialize};
use std::fmt;
use std::fmt::{Formatter, Error};
use hex::ToHex;
use chrono::{Utc};

#[derive(Serialize, Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub body: String,
    pub source: String,
    pub url: String,
    pub time_received: String,
}

impl NewsItem {
    pub fn create(title: String, body: String, source: String, url: String) -> NewsItem {
        NewsItem {
            title: String::from(title),
            body: String::from(body),
            source: String::from(source),
            url: String::from(url),
            time_received: Utc::now().to_rfc3339(),
        }
    }

    pub fn get_hash(&self) -> String {
        let news_str = format!("{}::{}::{}", self.title, self.body, self.source);
        let digest = md5::compute(news_str.as_bytes());
        let hex: String = digest.to_ascii_lowercase().as_slice().encode_hex();
        return hex;
    }

    pub fn as_json(&self) -> Result<String, String> {
        if let Ok(json_str) = serde_json::to_string(&self) {
            return Ok(json_str)
        } else {
            return Err(format!("could not serialize NewsItem {} to JSON string", &self))
        }
    }

    pub fn from_json(json_str: &str) -> Result<NewsItem, String> {
        if let Ok(parsed) = serde_json::from_str::<NewsItem>(json_str) {
            return Ok(parsed)
        } else {
            return Err(format!("could not deserialize NewsItem from JSON string {}", json_str))
        }
    }
}

impl fmt::Display for NewsItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[({} @ {}) {}: {}]", self.source, self.time_received, self.title, self.body)
    }
}

///
/// Takes in a string from rss feed and strips away all html from it (kind of)
///
pub fn cut_html(raw_str: &str) -> String {
    // FIXME: this is obviously not adequate and should be improved to handle all cases
    if let Some(index) = raw_str.find("<div class") {
        return String::from(&raw_str[..index])
    } else {
        return String::from(raw_str)
    }
}

///
/// TODO: use url module to extract the name here
///
/// http://www.france24.com/en/france/rss => france24.com
/// https://www.channelnewsasia.com/rssfeeds/8395838 => channelnewsasia.com
/// http://feeds.skynews.com/feeds/rss/technology.xml => feeds.skynews.com
/// https://elpais.com/tag/rss/europa/a/ => elpais.com
///
pub fn extract_source_name(url: &str) -> String {
    let trimmed = url
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .trim_start_matches("www.")
        .trim();
    if let Some(index) = trimmed.find("/") {
        return String::from(&trimmed[..index]);
    } else {
        return String::from(trimmed)
    }
}