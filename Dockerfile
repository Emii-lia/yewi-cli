FROM rust:1.94.0 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/yewi /app/yewi
CMD ["/app/yewi"]

FROM rust:1.94.0 as tester
WORKDIR /app
COPY . .
RUN cargo test --release