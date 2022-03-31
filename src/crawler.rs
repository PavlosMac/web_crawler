use super::errors::*;
use super::errors::*;
use super::HashMap;
use futures::{stream, FutureExt, StreamExt, TryStreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;
use tokio::io::AsyncWriteExt;

use std::{
    io::{self, Write},
    iter,
};

use crate::errors::RError;

type Db = Arc<Mutex<HashMap<String, String>>>;

#[derive(Debug)]
pub struct Crawler {
    pub(crate) results: Db,
    pub(crate) link_count: usize,
    links: Vec<String>,
}


// impl Crawler {
// pub fn new(links: Vec<String>) -> Self {
//     Self {
//         links,
//         link_count: 0,
//         results: Arc::new(Mutex::new(HashMap::new())),
//     }
// }

struct Req {
    body: String,
}

impl Req {
    async fn resolve(&self) -> Result<String, RError> {
        let res = reqwest::get(self.body.clone()).await?;
        Ok(res.status().to_string())
        // match res {
        //     Ok(r) => r.status().to_string(),
        //     Err(e) => Err(RError::Reqwest)
        // }
    }
}

pub async fn crawl(links: Vec<String>) {

    let tasks: Vec<_> = links
        .into_iter()
        .map(|mut item| {
            tokio::spawn(async {
                let res = resolve(&item).await;
                item.push_str(" - ");
                if res.is_ok() {
                    item.push_str(&res.unwrap());
                } else {
                    item.push_str("Request failed.")
                }
                item
            })
        })
        .collect();

    let mut items = vec![];
    for task in tasks {
        items.push(task.await.unwrap());
    }
    println!("{:?}", items);
}

async fn resolve(s: &str) -> Result<String, RError> {
    let res = reqwest::get(s).await?;
    Ok(res.status().to_string())
}
//
// async fn process_link(&mut self, url: &str) -> reqwest::StatusCode {
//     let res = reqwest::get(url).await;
//     match res {
//         Ok(res) => {
//             println!("Checking... {}", &url);
//             return res.status();
//         },
//         Err(_) => panic!("Possible network error {}", res.unwrap().status()),
//     };
// }
// }


#[cfg(test)]
mod tests {
    use super::super::*;
    use std::str::FromStr;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_check_link() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("/");
            then.status(200).body("");
        });

        let mock1 = Url::from_str("https://blog.x5ff.xyz/blog/async-tests-tokio-rust/");
        let mock2 = Url::from_str("https://blog.x5ff.xyz/blog/async-tests-tokio-rust/error");

        assert_eq!(aw!(check_link(&mock1.unwrap())).unwrap(), true);
        assert_eq!(aw!(check_link(&mock2.unwrap())).unwrap(), false);
    }

    #[test]
    fn test_check_link_error() {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("**/error");
            then.status(400).body("");
        });

        let mock = Url::from_str("https://blog.x5ff.xyz/mockerror/error");

        assert_eq!(aw!(check_link(&mock.unwrap())).unwrap(), false);
    }

    #[test]
    fn test_handle_async_tasks() {
        let mut urls = HashSet::new();

        urls.insert(Url::from_str("https://blog.x5ff.xyz/blog/async-tests-tokio-rust/").unwrap());
        urls.insert(Url::from_str("https://www.cloudsavvyit.com/10271/understanding-the-docker-build-context-why-you-should-use-dockerignore/").unwrap());
        urls.insert(Url::from_str("https://docs.docker.com/storage/volumes/").unwrap());
        urls.insert(
            Url::from_str("https://blog.sedrik.se/posts/my-docker-setup-for-rust/").unwrap(),
        );
        urls.insert(
            Url::from_str("https://blog.sedrik.se/posts/my-docker-setup-for-rust/").unwrap(),
        );

        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(GET).path("");
            then.status(200).body("");
        });

        assert_eq!(aw!(handle_async_tasks(urls)).unwrap().keys().len(), 4);
    }
}
