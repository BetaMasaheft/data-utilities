FROM messense/rust-musl-cross:x86_64-musl AS builder

COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --bin data-utilities --release --target-dir / --target x86_64-unknown-linux-musl

RUN ls -l /x86_64-unknown-linux-musl/release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y curl zip unzip default-jre libsaxonb-java libc6

COPY --from=builder /x86_64-unknown-linux-musl/release/data-utilities /

LABEL org.opencontainers.image.source=https://github.com/BetaMasaheft/data-utilities
LABEL org.opencontainers.image.description="Docker container for BM data utilities"
LABEL org.opencontainers.image.licenses=MIT
LABEL org.opencontainers.image.authors="Claudius Teodorescu <claudius.teodorescu@gmail.com>"

# sudo docker build --progress=plain --no-cache -t data-utilities .
# sudo docker run -a stderr data-utilities:latest
