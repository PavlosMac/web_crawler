FROM rust:latest

WORKDIR /web_crawler

COPY . .

RUN cargo install --path .

RUN cargo build --release

ENTRYPOINT ["/web_crawler/target/release/crw"]
