#![allow(unused)]
// // mod error;
//
// // use error::CrawlerError;
use reqwest;
use std::env;
use url::{Host, Url};
// use std::error::Error;
// use std::fs::File;
//
// use std::fmt::Display;
//

use select::document::Document;
use select::predicate::{Name, Predicate};
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Parse(::url::ParseError);
            ReqError(reqwest::Error);
        }
    }
}
use errors::*;

#[derive(Debug)]
struct Domain {
    base: String,
    pub indexables: Vec<String>,
}

impl Domain {
    /// create url request string and new struct - can unwrap host, because args has already been parsed
    pub fn new(args: &[String]) -> Result<Self> {
        let args: Vec<String> = env::args().collect();
        let mut arg = args[1].clone();
        let domain = Url::parse(&arg)?;
        let origin = domain.host().unwrap();
        let mut u = String::from("https://");
        u.push_str(&origin.to_string());

        Ok(Self {base: u, indexables: Vec::new()})
    }

    pub async fn process_domain_links(&mut self) -> Result<()> {
        let origin  = self.base.clone();
        let formed = {
            if !origin.contains("https://") {
                let mut u = String::from("https://");
                u.push_str(&origin.to_string());
                u
            } else {
                origin
            }
        };
        let res = reqwest::get(formed).await?.text().await?;
        let links: HashSet<String> = Document::from(res.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|n| n.to_owned())
            .collect::<HashSet<String>>();
        self.parse_links(links);
        Ok(())
    }

    fn parse_links(&mut self, links: HashSet<String>) {
        for link in links {
            println!("{}", link);
            if link.starts_with("/") {
                let full_u = format!("{}{}", &self.base, &link);
                self.indexables.push(full_u);
            }
            if link.contains(&self.base) {
                self.indexables.push(link)
            }
        }
    }
}

#[tokio::main]
pub async fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut d = Domain::new(&args)?;
    d.process_domain_links().await;
    let findings = d.indexables;
    println!("{:?}", findings);
    Ok(())
}
