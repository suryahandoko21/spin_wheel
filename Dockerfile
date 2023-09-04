FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools musl-dev wget gcc libssl-dev pkg-config libwayland-cursor0 libwayland-dev
RUN update-ca-certificates
RUN apt-get -y install libssl-dev pkg-config


ENV USER=rust
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"



WORKDIR /spin-wheel

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine

RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*


COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /spin-wheel

COPY --from=builder /spin-wheel/target/release/spin-wheel ./

#RUN apt-get update && apt-get install -y wget libpq5 libssl-dev gcc libgcc1 libc6
#COPY --from=builder /spin-wheel/target/release/spin-wheel /usr/local/bin

USER rust:rust

ENTRYPOINT ["./spin-wheel"]

