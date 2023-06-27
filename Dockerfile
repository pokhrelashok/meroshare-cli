FROM rust:1.61 AS builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder ./target/release/cli ./target/release/cli
COPY --from=builder ./users.json ./users.json
CMD ["/target/release/cli"]
