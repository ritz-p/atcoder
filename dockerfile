FROM rust:1.42.0

RUN apt update
RUN apt install vim -y
WORKDIR /workspace
ENV RUST_BACKTRACE=1