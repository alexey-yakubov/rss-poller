extern crate rss_poller;

use rss_poller::domain::{NewsItem, cut_html, extract_source_name};
use rss_poller::config::Config;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use serde_json;

#[test]
fn test_news_item_serialization_deserialization() {
    let news_item = NewsItem::create(
        String::from("title foo"),
        String::from("body bar"),
        String::from("source baz"),
        String::from("link kadabra"));

    let serialized = serde_json::to_string(&news_item).unwrap();

    if let Ok(deserialized) = serde_json::from_str::<NewsItem>(&serialized) {
        assert_eq!("title foo", deserialized.title);
        assert_eq!("body bar", deserialized.body);
        assert_eq!("source baz", deserialized.source);
        assert_eq!("link kadabra", deserialized.url);
    } else {
        assert!(false, "deserialization was not successful");
    }
}

#[test]
fn test_news_item_get_hash() {
    let news_item = NewsItem::create(
        String::from("title foo"),
        String::from("body bar"),
        String::from("source baz"),
        String::from("link kadabra"));
    let md5_hash = news_item.get_hash();
    assert_eq!(md5_hash, "908dbe8c37fc78378a149ef3787728f7");
}

#[test]
fn test_cut_html() {
    let raw_str = r#"Supreme Leader Ayatollah Ali Khamenei told Iran's elite Revolutionary Guards on Sunday to develop more advanced and modern weapons, the semi-official Tasnim news agency reported, amid rising regional tensions.<div class="feedflare">
<a href="http://feeds.reuters.com/~ff/Reuters/worldNews?a=-hevouWjzCg:qBtmTVqIiX4:yIl2AUoC8zA"><img src="http://feeds.feedburner.com/~ff/Reuters/worldNews?d=yIl2AUoC8zA" border="0"></img></a> <a href="http://feeds.reuters.com/~ff/Reuters/worldNews?a=-hevouWjzCg:qBtmTVqIiX4:F7zBnMyn0Lo"><img src="http://feeds.feedburner.com/~ff/Reuters/worldNews?i=-hevouWjzCg:qBtmTVqIiX4:F7zBnMyn0Lo" border="0"></img></a> <a href="http://feeds.reuters.com/~ff/Reuters/worldNews?a=-hevouWjzCg:qBtmTVqIiX4:V_sGLiPBpWU"><img src="http://feeds.feedburner.com/~ff/Reuters/worldNews?i=-hevouWjzCg:qBtmTVqIiX4:V_sGLiPBpWU" border="0"></img></a>
</div><img src="http://feeds.feedburner.com/~r/Reuters/worldNews/~4/-hevouWjzCg" height="1" width="1" alt=""/>"#;

    let expected_str = r#"Supreme Leader Ayatollah Ali Khamenei told Iran's elite Revolutionary Guards on Sunday to develop more advanced and modern weapons, the semi-official Tasnim news agency reported, amid rising regional tensions."#;

    assert_eq!(cut_html(raw_str), String::from(expected_str));
}

#[test]
fn test_extract_source_name() {
    assert_eq!(
        extract_source_name("http://feeds.reuters.com/news/artsculture"),
        "feeds.reuters.com");
    assert_eq!(
        extract_source_name("http://newsrss.bbc.co.uk/rss/newsonline_uk_edition/front_page/rss.xml"),
        "newsrss.bbc.co.uk");
    assert_eq!(
        extract_source_name("https://lenta.ru/rss"),
        "lenta.ru");
    assert_eq!(
        extract_source_name("https://news.ycombinator.com/rss"),
        "news.ycombinator.com");
    assert_eq!(
        extract_source_name("http://rss.cnn.com/rss/edition_world.rss"),
        "rss.cnn.com");
    assert_eq!(
        extract_source_name("https://www.cnbc.com/id/100003114/device/rss/rss.html"),
        "cnbc.com");
    assert_eq!(
        extract_source_name("http://ep00.epimg.net/rss/elpais/portada.xml"),
        "ep00.epimg.net");
    assert_eq!(
        extract_source_name("https://elpais.com/tag/rss/latinoamerica/a/"),
        "elpais.com");
    assert_eq!(
        extract_source_name("http://estaticos.elmundo.es/elmundo/rss/valencia.xml"),
        "estaticos.elmundo.es");
    assert_eq!(
        extract_source_name("http://www.france24.com/en/france/rss"),
        "france24.com");
    assert_eq!(
        extract_source_name("http://rss.dw.com/xml/rss_en_science"),
        "rss.dw.com");
    assert_eq!(
        extract_source_name("http://rss.dw.com/xml/rss_en_science"),
        "rss.dw.com");
    assert_eq!(extract_source_name(""), "");
    assert_eq!(extract_source_name("   "), "");
    assert_eq!(extract_source_name(" foo bar  "), "foo bar");
}