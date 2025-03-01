FROM rust:latest

RUN apt update && \
    apt install vim clang cmake libssl-dev build-essential -y
RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch taplo-cli && \
    rustup install nightly
WORKDIR /workspace
COPY Cargo.toml .
RUN sed -i 's/^members = \[.*\]/members = \["contest_prepare"\]/' Cargo.toml
COPY Cargo.lock .
COPY contest_prepare contest_prepare
RUN cargo build --release --bin prepare
RUN cp target/release/prepare /usr/bin/prepare
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1