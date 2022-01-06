FROM rust:1.50 as builder
WORKDIR /build
COPY . .
RUN rustup component add rustfmt
RUN cargo install --bin server --path .

FROM photon:latest
EXPOSE 8000
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
CMD ["server"]