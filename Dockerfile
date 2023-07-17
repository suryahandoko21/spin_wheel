# ####################################################################################################
# ## Builder
# ####################################################################################################
# FROM rust:latest AS builder




# WORKDIR /spin

# COPY ./ .
# RUN dir -s    

# # We no longer need to use the x86_64-unknown-linux-musl target
# RUN cargo build --release

# ####################################################################################################
# ## Final image
# ####################################################################################################

# WORKDIR /spin

# # Copy our build
# COPY --from=builder /spin/target/release/spin-wheel ./
# RUN ls -a

# # Use an unprivileged user.


# ENTRYPOINT ["./spin-wheel"]


FROM rust:latest as builder
WORKDIR /usr/src/spin-wheel
COPY . .
RUN cargo install --path .
# RUN cargo install diesel_cli --no-default-features --features postgres
FROM debian
# RUN 

RUN apt-get update & apt-get install -y extra-runtime-dependencies & apt-get install libpq5 -y libpq libpq-dev  postgresql-client-13 & rm -rf /var/lib/apt/lists/*
# RUN cargo install diesel_cli --no-default-features --features postgres
COPY --from=builder /usr/local/cargo/bin/spin-wheel /usr/local/bin/spin-wheel
CMD ["spin-wheel"]