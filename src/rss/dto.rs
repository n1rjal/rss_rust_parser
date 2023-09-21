use super::errors::RssParsingError;
use chrono;
use reqwest;
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rss {
    channel: Channel,
}

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    #[serde(default)]
    title: String,

    #[serde(default)]
    description: String,

    #[serde(default)]
    image: Option<Image>,

    #[serde(default)]
    item: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    #[serde(default)]
    url: String,

    #[serde(default)]
    title: String,

    #[serde(default)]
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub link: String,

    #[serde(default)]
    pub publish_date: String,
}

impl Rss {
    /// The `parse` function in Rust is used to parse an RSS feed from a given URL and return the parsed
    /// data as a `Result` of type `Rss` or an `RssParsingError`.
    ///
    /// Arguments:
    ///
    /// * `url`: The `url` parameter is a `String` that represents the URL of the RSS feed that you want
    /// to parse.
    ///
    /// Returns:
    ///
    /// The function `parse` returns a `Result` type with the success case containing an `Rss` object
    /// and the error case containing an `RssParsingError`.
    pub async fn parse(url: String) -> Result<Rss, RssParsingError> {
        let res = reqwest::Client::new().get(url).send().await?;
        let text = &res.text().await?;

        let rss = serde_xml_rs::from_str::<Rss>(text)?;
        println!("\n===============================");
        print!("{}\n", rss.channel.title);
        print!("{}\n", rss.channel.description);
        print!("Found {} contents.\n", rss.channel.item.len());
        println!("================================\n");
        Ok(rss)
    }

    /// The function `get_items` returns a reference to a vector of items after parsing and formatting
    /// their publish dates.
    ///
    /// Returns:
    ///
    /// a mutable reference to a vector of items (`&'c Vec<Item>`).
    pub fn get_items<'c>(&'c mut self) -> &'c Vec<Item> {
        for item in self.channel.item.iter_mut() {
            match chrono::DateTime::parse_from_rfc2822(&item.publish_date) {
                Ok(date) => date.format("%Y-%m-%d %H:%M:%S").to_string(),
                Err(..) => String::from(""),
            };
        }
        &self.channel.item
    }
}

/// The `impl Clone for Item` block is implementing the `Clone` trait for the `Item` struct. This allows
/// instances of `Item` to be cloned, creating a new instance with the same values as the original.
impl Clone for Item {
    fn clone(&self) -> Self {
        Item {
            title: self.title.clone(),
            link: self.link.clone(),
            publish_date: self.publish_date.clone(),
        }
    }
}
