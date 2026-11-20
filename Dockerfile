# syntax = docker/dockerfile:1.2
FROM rust:bullseye AS builder

RUN apt-get update && apt-get install -y default-jre libsaxonb-java libc6

# Download and compile the Rust executable for the conceptual mapper
WORKDIR /home/work/conceptual-mapper
RUN curl https://gitlab.com/claudius-teodorescu/conceptual-mapper/-/package_files/335497958/download -o conceptual-mapper-bin && chmod +x conceptual-mapper-bin

RUN cargo build -q --bin bm-data-service --target-dir ./target --release --target x86_64-unknown-linux-gnu &&\
    cp ./target/x86_64-unknown-linux-gnu/release/bm-data-service /home/ubuntu


LABEL maintainer Claudius Teodorescu <claudius.teodorescu@gmail.com>

# sudo docker build -t data-utilities .
