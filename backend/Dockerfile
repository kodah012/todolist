FROM rust:latest

WORKDIR /backend

COPY . .

RUN cargo build --profile=release

ENTRYPOINT ["cargo", "run", "--profile=release"]