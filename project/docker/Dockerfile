# NB: This is not a production-grade Dockerfile.
#
##################
### build stage ##
##################
#FROM rust:1-slim-bookworm AS builder
#WORKDIR /code
#
## Download crates-io index and fetch dependency code.
## This step avoids needing to spend time on every build downloading the index
## which can take a long time within the docker context. docker will cache it.
#RUN USER=root cargo init
#COPY ../../Cargo.toml Cargo.toml
#RUN cargo fetch
#
## copy app files
#COPY ../../src src
#
## compile app
#RUN cargo build --release

###############
## run stage ##
###############
FROM bitnami/minideb:bookworm
WORKDIR /app

# Install required dependencies (OpenSSL)
RUN install_packages libssl3

# Install ca-certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the .env file
COPY /config /etc/config

# copy server binary from build stage
#COPY --from=builder /code/target/release/med-bot med-bot
COPY /target/release/med-bot /app/med-bot

## set user to non-root unless root is required for your app
USER 1001

## indicate what port the server is running on
EXPOSE 8082

# run server
CMD ["/app/med-bot"]