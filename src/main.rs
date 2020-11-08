use std::collections::HashSet;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Write, Read, BufReader};

use rss::Channel;

use crate::config::Config;
use crate::domain::{NewsItem, cut_html, extract_source_name};
use chrono::prelude::*;

pub mod domain;
pub mod config;


/// 1. Load config, provided via command line argument
/// 2. Load already saved news for today from [output] folder
/// 3. Calculate MD5 hash for each news item and persist to a set to check for duplicates
/// 4. Go via each link and persist news to file
fn main() {
    let args: Vec<String> = env::args().collect();

    // E.g. arguments are: ["target/debug/rss-poller", "/home/alexey/code/rss-poller/cfg/config.toml"]
    if args.len() != 2 {
        println!("Usage: rss-poller [config.toml]");
        println!("Actual args: {:?}", args);
        return;
    }

    let cfg_path = &args[1];
    match Config::load_from_file(cfg_path) {
        Ok(config) => {
            println!("Loaded config: {:#?}", config);
            do_processing(config);
        }
        Err(e) => {
            println!("Could not load config: {}", e);
        }
    }
}

fn do_processing(config: Config) {
    let mut news_hashes = get_news_hashes_from_dir(&config.output.path);
    println!("Overall news hashes loaded: {}", news_hashes.len());
    poll_data(&config.input.links, &config.output.path, &mut news_hashes);
}

fn get_file_list_in_dir(path: &str) -> Vec<String> {
    let mut result = Vec::new();
    match fs::read_dir(path) {
        Err(why) => {
            println!("Could not find any files in {}, reason: {:?}", path, why.kind())
        }
        Ok(paths) => {
            for path in paths {
                let dir_entry = path.unwrap();
                let whole_path = dir_entry.path().into_os_string().into_string().unwrap();
                if whole_path.ends_with(".json") {
                    result.push(whole_path);
                }
            }
        }
    }

    return result;
}

fn get_news_hashes_from_dir(path: &str) -> HashSet<String> {
    println!("Retrieving hashes from dir: {}", path);
    let mut result = HashSet::new();

    let files = get_file_list_in_dir(path);
    for file in files {
        for hash in get_news_hashes_from_file(&file) {
            result.insert(hash);
        }
    }

    return result
}

fn get_news_hashes_from_file(path: &str) -> HashSet<String> {
    println!("Retrieving hashes from file: {}", path);

    let result = HashSet::new();

    if let Ok(news_file) = OpenOptions::new().read(true).open(&path) {
        let mut buffered = BufReader::new(news_file);
        let mut lines = String::new();
        buffered.read_to_string(&mut lines);
        return get_news_hashes_from_str(lines);
    } else {
        println!("Could not open file to compute hashes: {}", path);
    }

    println!("Found {} recent and unique NewsItems", result.len());
    return result;
}

fn get_news_hashes_from_str(lines: String) -> HashSet<String> {
    let mut result = HashSet::new();
    for line in lines.lines() {
        // TODO: remove unwrawp and handle the error condition
        let news_item = NewsItem::from_json(line).unwrap();
        let hash = news_item.get_hash();
        result.insert(hash);
    }
    return result;
}

fn poll_data(urls: &Vec<String>, output_path: &str, existing_hashes: &mut HashSet<String>) {
    let output_file_name = get_file_path(output_path, "news");
    println!("Writing news to: {}", output_file_name);

    if let Ok(mut output_file) = OpenOptions::new().write(true).create(true).append(true).open(&output_file_name) {
        for i in 0..urls.len() {
            let url = urls.get(i).unwrap();
            println!("Fetching data [{}/{}] from: {}", i + 1, urls.len(), url);
            let news_items = fetch_news_items(url);

            for news_item in news_items.iter() {
                let news_item_hash = news_item.get_hash();
                if !existing_hashes.contains(&news_item_hash) {
                    if let Ok(json) = news_item.as_json() {
                        writeln!(&mut output_file, "{}", json);
                        existing_hashes.insert(news_item_hash);
                    }
                }
            }
        }
    } else {
        println!("Could not create file for writing: {}", output_file_name);
    }
}

fn fetch_news_items(url: &str) -> Vec<NewsItem> {
    let channel = Channel::from_url(url).unwrap();

    let mut result = Vec::new();
    for item in channel.items() {
        let title: String = String::from(item.title().unwrap_or(""));
        let body = cut_html(item.description().unwrap_or(""));
        let link: String = String::from(item.link().unwrap_or(""));
        let source = extract_source_name(&link);

        let news_item = NewsItem::create(title, body, source, link);
        result.push(news_item);
    }

    return result;
}

fn get_file_path(dir: &str, prefix: &str) -> String {
    let now: DateTime<Utc> = Utc::now();
    let today = now.format("%Y-%m-%d").to_string();
    if dir.ends_with("/") {
        return format!("{}{}-{}{}", dir, prefix, today, ".json");
    } else {
        return format!("{}/{}-{}{}", dir, prefix, today, ".json");
    }
}