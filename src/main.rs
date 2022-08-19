use std::{error::Error};
use rss::{Channel};
use voca_rs::Voca;
use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
struct Article {
    title: String,
    description: String,
    content: String,
    date: DateTime<FixedOffset>,
}

async fn read_feed(url: String) -> Result<Vec<Article>, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    //Ok(channel.items)
    let articles:Vec<Article> = channel.items
        .into_iter()
        .map(|it| Article {
            title: it.title.unwrap(),
            description: it.description.unwrap()._strip_tags(),
            content: it.content.unwrap()._strip_tags(),
            date: DateTime::parse_from_rfc2822(&it.pub_date.unwrap()).unwrap(),
        }).collect();
        Ok(articles)
}

async fn collect_articles_from_list(list_url: Vec<&str>) -> Result<Vec<Article>, Box<dyn Error>> {
    let mut articles = Vec::new();
    for url in list_url{
        articles.extend(read_feed(url.to_string()).await?)
    }
    articles.sort_by(|a,b| b.date.cmp(&a.date));
    Ok(articles)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let list_url = vec!["https://hackaday.com/blog/feed/", "http://darmont.free.fr/?feed=rss2"];
    let articles = collect_articles_from_list(list_url).await?;
    for item in articles{
        println!("{:#?}", item);
    }
    Ok(())
}