FROM rustlang/rust:nightly-alpine

ENV NEXT_TELEMETRY_DISABLED=1

RUN apk update && \
    apk add \
        nodejs \
        npm \
        openssl \
        musl-dev \
        gcc \
        g++ \
        binutils \
        make \
        autoconf \
        automake \
        alpine-sdk \
        openssl-dev \
        openssl-libs-static \
        libssl3 \
        libssl1.1 \
        libcrypto3 \
        libcrypto1.1 \
        pkgconfig

RUN npm install --global pnpm@latest

ADD . /usr/src/app
WORKDIR /usr/src/app

RUN pnpm install
RUN pnpm -C apps/client build

RUN cargo build --release --target $(uname -m)-unknown-linux-musl
RUN cp target/$(uname -m)-unknown-linux-musl/release/server /usr/local/bin/dragon

CMD [ "/usr/local/bin/dragon" ]
