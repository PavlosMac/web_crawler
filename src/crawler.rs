use crate::errors::RError;
use super::info;

/// take the domain links, spawn tasks to make requests, collect the urls with new format, appending status code or failed request
pub async fn crawl(links: Vec<String>) -> Result<Vec<String>, RError> {
    let tasks: Vec<_> = links
        .into_iter()
        .map(|mut item| {
            tokio::spawn(async {
                let res = resolve(&item).await;
                if let Ok(code) = &res {
                    let f = format!(" -- {}", &code.as_str());
                    item.push_str(f.as_str());
                } else {
                    item.push_str(" --- failed to index link");
                }
                item
            })
        })
        .collect();

    let mut items = vec![];
    for task in tasks {
        items.push(task.await?);
    }
    info!("count: {}", &items.len());
    Ok(items)
}
/// resolve the http request to the domain, print to stderr if error
async fn resolve(s: &str) -> Result<reqwest::StatusCode, RError> {
    info!("Indexing.... {}", &s);
    let res = reqwest::get(s).await;
    return match res {
        Ok(r) => Ok(r.status()),
        Err(e) => {
            eprintln!("{}",e);
            Err(RError::InvalidHttpResponse("Bad request".to_owned()))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)};
    }

    /// test that N requests are made to mock http server, use blocking macro for async context
    #[tokio::test]
    async fn test_crawl() {
        let mut urls = Vec::new();
        urls.push(String::from("https://www.meh.com/10271/understanding-the-docker-build-context-why-you-should-use-dockerignore/"));
        urls.push(String::from("https://www.cloudsavvyit.com/10271/understanding-the-docker-build-context-why-you-should-use-dockerignore/"));

        let result = crawl(urls).await;
        let server = MockServer::start();

        server.mock(|when, then| {
            when.path("");
            then.status(200).body("<body>");
        });

        assert_eq!(result.unwrap().len(), 2);
    }
}
