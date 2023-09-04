FROM rust:latest AS builder

RUN apt-get update && apt-get install -y musl-tools musl-dev wget gcc libssl-dev pkg-config libwayland-cursor0 libwayland-dev
RUN update-ca-certificates
RUN apt-get -y install libssl-dev pkg-config

WORKDIR /spin-wheel

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim AS runtime

RUN apt-get update && apt-get install -y wget libpq5 libssl-dev gcc libgcc1

# Import from builder.
COPY --from=builder /spin-wheel/target/release/spin-wheel /usr/local/bin


CMD ["/usr/local/bin//spin-wheel"]

