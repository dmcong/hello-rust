FROM rust:lastest

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# ENV ROCKET_ADDRESS 0.0.0.0
EXPOSE 8000
CMD ./target/release/hello-rust