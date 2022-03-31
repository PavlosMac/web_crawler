use super::errors;
use super::*;

use select::{
    document::Document,
    predicate::{Name, Predicate},
};

#[derive(Debug)]
pub struct Domain {
    base: String,
    pub indexables: Vec<String>,
}

impl Domain {
    /// create url request string and new struct - can unwrap host, because args has already been parsed
    pub fn new(args: &[String]) -> Result<Self, RError> {
        let mut arg = args[1].clone();
        let domain = Url::parse(&arg)?;
        let origin = domain.host().unwrap();
        let mut u = String::from("https://");
        u.push_str(&origin.to_string());
        Ok(Self {
            base: u,
            indexables: Vec::new(),
        })
    }
    ///
    pub async fn process_domain_links(&mut self) -> Result<(), RError> {
        let origin = self.base.clone();
        let formed = {
            if !origin.contains("https://") {
                let mut u = String::from("https://");
                u.push_str(&origin.to_string());
                u
            } else {
                origin
            }
        };
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
