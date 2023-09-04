FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools musl-dev wget gcc libssl-dev pkg-config libwayland-cursor0 libwayland-dev
RUN update-ca-certificates
RUN apt-get -y install libssl-dev pkg-config

WORKDIR /spin-wheel

COPY ./ .

RUN cargo build --release --bin spin-wheel

####################################################################################################
## Final image
####################################################################################################
FROM alpine

RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*

WORKDIR /spin-wheel

COPY --from=builder /spin-wheel/target/release/spin-wheel ./

#RUN apt-get update && apt-get install -y wget libpq5 libssl-dev gcc libgcc1 libc6
#COPY --from=builder /spin-wheel/target/release/spin-wheel /usr/local/bin


ENTRYPOINT ["./spin-wheel"]

