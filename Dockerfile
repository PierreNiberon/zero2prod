# command to build the image
# docker build --tag zero2prod --file Dockerfile .
# We use the latest Rust stable release as base image
FROM rust:1.65.0-buster
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
#Set the sqlx offline mode to use the sqlx-data.json
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
# we also remove the src files that are not needed anymore.
RUN cargo build --release & rm src/*.rs
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]
