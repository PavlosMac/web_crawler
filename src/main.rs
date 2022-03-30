use url::{Url, Position, ParseError};
use clap::Parser;
use clap::{Arg};
use std::env;
use reqwest;
use std::collections::HashSet;
use select::document::Document;
use select::predicate::{Name, Predicate};
use std::path::Path;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let req_url = "https://".to_owned() + &*args[1].to_string();
    let url = Url::parse(&req_url).unwrap();
    let res = reqwest::get(url.as_ref()).await;

    let res = res.unwrap().text().await;
    // println!("{}", res.status_code);
    // println!("{}", res.http_version);
    let res = res.unwrap();

    let links: HashSet<String> = Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(|n| n.to_owned())
        .collect::<HashSet<String>>();

    // println!("{:?}", links);
    let mut new_list = vec![];
    let domain = req_url;
    for link in links {
        if link.starts_with("/") {
            let full_u = format!("{}{}", &domain, &link);
            new_list.push(full_u);
        }
        if link.contains(&domain) {
            new_list.push(link)
        }
    }

    println!("{:?}", new_list);
}
