# Use the official Rust image as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /usr/src/myapp

# Copy the current directory contents into the container at /usr/src/myapp
COPY ./app .

# Compile the project
RUN cargo build --release

# Copy the statically-linked binary into a bare minimum image
FROM debian:buster-slim
COPY --from=0 /usr/src/myapp/target/release/actix-mongo-api /usr/local/bin/myapp

# Run the binary
CMD ["myapp"]