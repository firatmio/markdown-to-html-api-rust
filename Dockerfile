FROM rust:1.72-slim as builder
WORKDIR /app


COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true


COPY . .
RUN rm -f target/release/deps/markdown_to_html_api* || true
RUN cargo build --release


FROM debian:bookworm-slim
COPY --from=builder /app/target/release/markdown_to_html_api /usr/local/bin/markdown_to_html_api
EXPOSE 8080
CMD ["/usr/local/bin/markdown_to_html_api"]