FROM rust:1.85.1-alpine3.21 AS builder
USER root
RUN apk add musl-dev # for crti for async-trait crate
WORKDIR /src
COPY . /src
RUN cargo build --release

FROM scratch
COPY --from=builder /src/target/release/get-checksum /app/
USER 65534
CMD ["/app/get-checksum"]
