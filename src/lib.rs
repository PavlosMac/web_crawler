#![allow(unused)]
use reqwest;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use url::{Host, Url};
mod crawler;
mod domain;
use domain::*;
mod errors;
use errors::*;
mod file;

#[tokio::main]
pub async fn run() -> Result<(), RError> {
    let args: Vec<String> = env::args().collect();
    let mut arg_1 = args[1].clone();
    let mut d = Domain::new(arg_1)?;
    d.process_domain_links().await?;
    let mut data = crawler::crawl(d.indexables).await?;
    let handler = tokio::task::spawn_blocking(move || {
        file::save_file(data, d.host);
    });

    handler.await?;
    Ok(())
}
