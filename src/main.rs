// #[macro_use]
// extern crate error_chain;
//
// mod errors {
//     error_chain!{
//         foreign_links {
//             Io(::std::io::Error);
//             Parse(::url::ParseError);
//         }
//     }
// }
// use errors::*;
//
// #[derive(Debug)]
// struct Domain {
//     base: Url,
//     indexables: Vec<String>
// }
//
// impl Domain {
//     pub fn new(args: &[String]) -> Result<Self> {
//         let args: Vec<String> = env::args().collect();
//         let arg = &args[1].as_ref();
//         let domain = Url::parse(arg)?;
//         Ok(Self { base: domain, indexables: Vec::new()})
//     }
//
//     // pub fn process_domain_links(&mut self)  {
//     // }
//
//     // fn do_http_request() {
//     //     let res = reqwest::get(url.as_ref()).await;
//     //     let res = res?.text().await;
//     //     let res = res?;
//     // }
// }

// fn run() -> Result<()> {
//     let args: Vec<String> = env::args().collect();
//     let d = Domain::new(&args);
//     println!("{:?}", d);
//
//     Ok(())
// }

fn main() {
    if let Err(e) = crawler::run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();
//     // get the input arg and validate it - should contain https
//     let req_url = "https://".to_owned() + &*args[1].to_string();
//     let url = Url::parse(&req_url).unwrap();
//     let res = reqwest::get(url.as_ref()).await;
//
//     let res = res.unwrap().text().await;
//     let res = res.unwrap();
//
//     let links: HashSet<String> = Document::from(res.as_str())
//         .find(Name("a"))
//         .filter_map(|n| n.attr("href"))
//         .map(|n| n.to_owned())
//         .collect::<HashSet<String>>();
//
//     let mut new_list = vec![];
//     let domain = req_url;
//     for link in links {
//         if link.starts_with("/") {
//             let full_u = format!("{}{}", &domain, &link);
//             new_list.push(full_u);
//         }
//         if link.contains(&domain) {
//             new_list.push(link)
//         }
//     }
//
//     println!("{:?}", new_list.len());
//     // create an async concurrent stream that does not block the main thread
//     let c: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
//     stream::iter(new_list.into_iter())
//         .for_each_concurrent(5, |url| {
//             let arc = c.clone();
//             async move {
//                 let res = reqwest::get(&url).await;
//                 match res {
//                     Ok(res) => {
//                         println!("Success! {}", &url);
//                         arc.lock().await.insert(url.to_string(), res.status().to_string());
//                     },
//                     _ => {
//                     panic!("Something unexpected happened.{}", res.unwrap().status());
//                     },
//                 };
//             }})
//         .await;
//
//     let guard = c.lock().await.clone();
// }
