FROM rust:1.79 AS builder
WORKDIR /builder
ADD . /builder
RUN cargo build --release
RUN chmod +x /builder/target/release/trust

FROM gcr.io/distroless/cc
WORKDIR /usr/src/trust
COPY --from=builder /builder/target/release/trust /usr/src/trust/run
COPY --from=builder /builder/src /usr/src/trust/src
ENTRYPOINT ["/usr/src/trust/run"]
