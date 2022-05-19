FROM rust:1.60 as builder
WORKDIR /usr/src/danbooru-tag-bot
RUN apt-get update && apt-get install libpq-dev -y
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev libpq-dev ca-certificates
COPY --from=builder /usr/local/cargo/bin/danbooru-tag-bot /usr/local/bin/danbooru-tag-bot
CMD ["danbooru-tag-bot", "bot"]