use invidious::{reqwest::blocking::Client, structs::hidden::SearchItem};
use std::error::Error;

pub struct MyVideo {
    pub title: String,
    pub id: String,
    pub length: u64,
}

pub fn search(q: &str) -> Result<Vec<MyVideo>, Box<dyn Error>> {
    let client = Client::new(String::from("https://vid.puffyan.us"));

    // search for videos with query q, filter results into struct MyVideo
    let results: Vec<MyVideo> = client
        .search(Some(format!("q={q}").as_str()))?
        .items
        .into_iter()
        .filter_map(|item| match item {
            SearchItem::Video {
                title, id, length, ..
            } => Some(MyVideo {
                title: title.trim().to_string(),
                id,
                length,
            }),
            _ => None,
        })
        .collect();

    Ok(results)
}
