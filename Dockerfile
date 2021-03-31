FROM rust:1.51

RUN cargo install cargo-audit
RUN cargo install cargo-criterion
RUN cargo install cargo-udeps
RUN cargo install xargo

RUN rustup component add clippy rustfmt
RUN rustup toolchain install nightly-2021-02-25 -c clippy -c rustfmt -c rust-src -c miri

WORKDIR /workdir
COPY . .

CMD [ "./check" ]
