# Async Web Crawler Implementation

Implementation of a subdomain crawler. This cli programme should:
* get all hosted links from single domain
* check hosted links for status code using concurrent tasks
* save the results along with totals to a file

Env args:
$arg1 = <url-to-crawl> 

```sh
> cargo run http://www.example-web-page.com
> cargo test
```

Can be run from docker container with docker-compose. Replace the second entrypoint argument in /web_crawler/docker-compose.yml.

```sh
entrypoint: ["/web_crawler/target/release/crw", "https://example.org"]
```
