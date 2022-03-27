FROM rust:latest
WORKDIR /app
COPY . /app
LABEL maintainer="Léopold Koprivnik Ibghy <skwal.net@gmail.com>"
RUN cargo build --release
ENTRYPOINT ["/app/target/release/linux-on-drugs"]
