# Async Web Crawler

Implementation of a domain crawler. This cli programme should:
* get all hosted links from single domain
* check unique links for status code using concurrent tasks
* save the results along with count to a file

Env args:
$arg = url


Note:
Requires `tmp` folder at project root for results file.


Run programme:
```sh
> RUST_LOG=info cargo run https://www.example-web-page.com
```

Run tests:
```sh
> cargo test
```


Can be run from docker container with docker-compose. Replace the second entrypoint argument in /web_crawler/docker-compose.yml.

With Docker:
Change second entrypoint arg in yml file.


`entrypoint: ["/web_crawler/target/release/crw", "https://example.org"]`

```sh
> docker-compose up
> docker-compose down
```
