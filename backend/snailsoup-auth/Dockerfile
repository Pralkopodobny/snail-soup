# syntax=docker/dockerfile:1

FROM rust:1.83 as builder
WORKDIR /usr/src/snailsoup

# Install host build dependencies.
RUN apt-get install libssl-dev pkg-config

ENV SQLX_OFFLINE=true

COPY . .

RUN cargo install --path .

FROM ubuntu:24.04
# RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/snailsoup /usr/local/bin/snailsoup
COPY ./migrations/ /usr/local/bin/migrations

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["snailsoup"]
