# Async Web Crawler Implementation

Implementation of a subdomain crawler. This cli programme should:
* get all hosted links from single domain
* check unique links for status code using concurrent tasks
* save the results along with count to a file

Env args:
$arg = url

Run programme:
```sh
> cargo run https://www.example-web-page.com
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
