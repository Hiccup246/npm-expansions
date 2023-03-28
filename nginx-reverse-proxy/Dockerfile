FROM rust:1.67.1 as builder

USER root

RUN cargo new --lib npm-expansions

WORKDIR /npm-expansions

COPY ./Cargo.toml ./Cargo.toml

ADD ./build.rs ./build.rs
ADD ./pages ./pages
ADD ./static ./static

RUN cargo build --release

FROM nginx:1.23.3 as reverse-proxy

COPY --from=builder /npm-expansions/target/static /etc/nginx/static
COPY --from=builder /npm-expansions/target/pages /etc/nginx/pages

COPY npm-expansions.conf /etc/nginx/nginx.conf