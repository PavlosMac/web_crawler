use url::{Url, Position, ParseError};
use clap::Parser;
use clap::{Arg};
use std::env;
use reqwest;
use std::collections::{HashSet, HashMap};
use select::document::Document;
use select::predicate::{Name, Predicate};
use std::path::Path;
use futures::{stream, StreamExt};
use tokio::sync::Mutex;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let req_url = "https://".to_owned() + &*args[1].to_string();
    let url = Url::parse(&req_url).unwrap();
    let res = reqwest::get(url.as_ref()).await;

    let res = res.unwrap().text().await;
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

    println!("{:?}", new_list.len());
    // create an async concurrent stream that does not block the main thread
    let c: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    stream::iter(new_list.into_iter())
        .for_each_concurrent(5, |url| {
            let arc = c.clone();
            async move {
                let res = reqwest::get(&url).await;
                match res {
                    Ok(res) => {
                        println!("Success! {}", &url);
                        arc.lock().await.insert(url.to_string(), res.status().to_string());
                    },
                    _ => {
                    panic!("Something unexpected happened.{}", res.unwrap().status());
                    },
                };
            }})
        .await;

    let guard = c.lock().await.clone();

    // println!("how many r's {}", guard.len());
    //
    // for (key,value) in &guard {
    //     println!("{}: {}", key, value);
    // }
}
