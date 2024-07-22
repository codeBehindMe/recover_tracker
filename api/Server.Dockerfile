FROM rust:1.79 as builder

RUN apt-get update && apt-get upgrade -y && apt-get install -y protobuf-compiler libprotobuf-dev

WORKDIR /app

COPY . .

RUN cargo build --bin server --release

RUN strip target/release/server

FROM debian:bookworm-slim as release

WORKDIR /app

COPY --from=builder /app/target/release/server .

EXPOSE 8080

ENTRYPOINT [ "./server" ]