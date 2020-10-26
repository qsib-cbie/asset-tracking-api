FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt update && \
    apt install -y gcc libzmq3-dev postgresql-client build-essential glances htop vim tree curl libpq-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /home/rustup.sh && \
    chmod +x /home/rustup.sh && \
    /home/rustup.sh -y && \
    . $HOME/.cargo/env && \
    echo ". $HOME/.cargo/env" >> $HOME/.bashrc

COPY migrations /home/app/migrations/
COPY src /home/app/src/
COPY diesel.toml Cargo.toml Cargo.lock .env /home/app/

RUN cd /home/app && . $HOME/.bashrc && \
    cargo build && cargo build --release