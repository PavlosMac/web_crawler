use super::errors;
use super::*;

use select::{
    document::Document,
    predicate::{Name, Predicate},
};

#[derive(Debug)]
pub struct Domain {
    pub base: String,
    pub indexables: Vec<String>,
}

const PROTOCOL: &str = "https://";

impl Domain {
    /// parse the input arg, create url request string and new struct - can unwrap host, because args has already been parsed
    pub fn new(input_arg: String) -> Result<Self, RError> {
        let domain = Url::parse(&input_arg)?;
        let origin = domain.host().unwrap();
        let mut u = String::from(PROTOCOL);
        u.push_str(&origin.to_string());
        Ok(Self {
            base: u,
            indexables: Vec::new(),
        })
    }
    /// request initial doc from domain, process href tags, return HashSet to ensure unique values
    pub async fn process_domain_links(&mut self) -> Result<(), RError> {
        let origin = self.base.clone();
        let formed = check_protocol(origin);
        let client = reqwest::Client::new();
        let res = reqwest::get(formed)
            .await?
            .text()
            .await?;
        let links: HashSet<String> = Document::from(res.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|n| n.to_owned())
            .collect::<HashSet<String>>();
        self.parse_links(links);
        Ok(())
    }
    /// loop links from domain, if link is path only, append domain
    fn parse_links(&mut self, links: HashSet<String>) {
        for link in links {
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
/// append protocol on origin for http client
fn check_protocol(org: String) -> String {
    if !org.contains(PROTOCOL) {
        let mut u = String::from(PROTOCOL);
        u.push_str(&org.to_string());
        return u
    }
    org
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn test_domain() {
        let arg_str = String::from("https://blog.com/blog/async-tests-tokio-rust/");
        let d = Domain::new(arg_str);
        assert!(d.is_ok());

        let arg_str = String::from("blog.com/----/");
        let d = Domain::new(arg_str);
        assert!(d.is_err());
    }

    #[test]
    fn test_check_protocol() {
        let arg_tes1 = String::from("https://blog.com");

        assert_eq!(check_protocol(String::from("https://blog.com")), arg_tes1);
        assert_eq!(check_protocol(String::from("blog.com")), arg_tes1);
    }

    // #[tokio::test]
    // async fn test_process_domain_links() {
    //     let arg_str = String::from("https://blog.com/blog/async-tests-tokio-rust/");
    //     let mut d = Domain::new(arg_str).unwrap();
    //
    //     let server = MockServer::start();
    //
    //     server.mock(|when, then| {
    //         when.method(GET).path("");
    //         then.status(200).body("");
    //     });
    //
    //     server.assert();
    //
    //     d.process_domain_links();
    // }
}
