FROM rust:1.42.0

RUN apt update && \
    apt install vim -y
WORKDIR /workspace
ENV USER=root
ENV RUST_BACKTRACE=1