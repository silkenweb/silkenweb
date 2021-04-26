FROM rust:1.51

RUN apt-get update && apt-get install -y curl

# For Cypress
RUN \
    apt-get update && \
    apt-get install -y libgtk2.0-0 libgtk-3-0 libgbm-dev libnotify-dev libgconf-2-4 libnss3 libxss1 libasound2 libxtst6 xauth xvfb
RUN \
    apt-get update && apt-get install -y npm

# Install NPM
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get update && apt-get install -y nodejs

# Install wasm-opt
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_101/binaryen-version_101-x86_64-linux.tar.gz | tar -xz
RUN cp binaryen-version_101/bin/wasm-opt /usr/bin/

RUN cargo install cargo-audit
RUN cargo install cargo-criterion
RUN cargo install cargo-udeps
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
RUN cargo install xargo

RUN rustup component add clippy rustfmt
RUN rustup target add wasm32-unknown-unknown
RUN rustup toolchain install nightly-2021-02-25 -c clippy -c rustfmt -c rust-src -c miri

WORKDIR /workdir
COPY . .

CMD [ "./check" ]
