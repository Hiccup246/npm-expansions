FROM rust:1.67.1 as builder

USER root

RUN cargo new --bin npm-expansions

WORKDIR /npm-expansions

COPY ./Cargo.toml ./Cargo.toml

ADD ./src ./src
ADD ./pages ./pages
ADD ./static ./static

RUN cargo build --release

RUN cargo run

FROM nginx:1.23.3 as reverse-proxy

COPY --from=builder /npm-expansions/minified_static /etc/nginx/static
COPY --from=builder /npm-expansions/minified_pages /etc/nginx/pages

ARG nginx_config

COPY ${nginx_config} /etc/nginx/nginx.conf