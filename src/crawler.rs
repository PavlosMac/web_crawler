use super::errors::*;
use super::errors::*;
use super::HashMap;
use futures::{stream, FutureExt, StreamExt, TryStreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::error::Error;
use crate::errors::RError;

pub async fn crawl(links: Vec<String>) -> Result<Vec<String>, RError> {
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
        items.push(task.await?);
    }
    println!("{:?}", items);
    Ok(items)
}

async fn resolve(s: &str) -> Result<String, RError> {
    let res = reqwest::get(s).await?;
    Ok(res.status().to_string())
}

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
