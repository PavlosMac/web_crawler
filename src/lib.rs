use std::collections::HashSet;
use std::env;
use url::Url;
mod crawler;
mod domain;
use domain::*;
mod errors;
use errors::*;
mod file;
use log::info;

#[tokio::main]
pub async fn run() -> Result<(), RError> {
    env_logger::init();
    info!("Run crawler..........");
    let args: Vec<String> = env::args().collect();
    let arg_1 = args[1].clone();
    let mut d = Domain::new(arg_1)?;
    d.process_domain_links().await?;

    let data = crawler::crawl(d.indexables).await?;
    let handler = tokio::task::spawn_blocking(move || {
        file::save_file(data, d.host).expect("no file created");
    });

    handler.await?;
    Ok(())
}
