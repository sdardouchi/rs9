FROM rust:latest as builder
WORKDIR /app
COPY . /app
RUN cargo build --bins -r 

FROM debian:bookworm-slim
RUN apt update
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        libffi-dev \
        libc6-dev &&\
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rs9 /usr/local/bin/rs9

CMD [ "rs9" ]
