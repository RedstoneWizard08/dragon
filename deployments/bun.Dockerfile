FROM rust:alpine

ARG ZIG_RELEASE=0.12.0-dev.1297+a9e66ed73
ARG BUN_RELEASE=bun-v1.0.9

RUN apk update && \
    apk add \
        curl \
        wget \
        tar \
        xz \
        git \
        alpine-sdk \
        clang \
        llvm \
        binutils \
        cmake \
        coreutils \
        sed \
        gnu-libiconv-dev \
        go \
        libtool \
        autoconf \
        automake \
        ninja \
        pkgconfig \
        ccache \
        lld \
        nodejs \
        npm \
        ruby \
        jq \
        bash

RUN mkdir -p /usr/local/zig
RUN curl -fsSLo /usr/local/zig/zig.tar.xz \
        "https://ziglang.org/builds/zig-linux-$(uname -m)-${ZIG_RELEASE}.tar.xz"

RUN tar -xJf /usr/local/zig/zig.tar.xz -C /usr/local/zig
RUN ln -sf "/usr/local/zig/zig-linux-$(uname -m)-${ZIG_RELEASE}" /usr/local/zig/current
RUN ln -sf /usr/local/zig/current/zig /usr/local/bin/zig

RUN npm install --global yarn

RUN echo 'echo $*' > /usr/bin/bun && \
    chmod +x /usr/bin/bun

RUN git clone \
        --recursive \
        --depth 1 \
        https://github.com/oven-sh/bun \
        -b "${BUN_RELEASE}" \
        /usr/src/bun

WORKDIR /usr/src/bun

RUN bash ./scripts/all-dependencies.sh
RUN yarn install
RUN make runtime_js fallback_decoder

RUN cd packages/bun-error && \
        yarn install && \
        yarn build

RUN cd src/node-fallbacks && \
        yarn install && \
        yarn build

RUN mkdir -p build
RUN rm -f build/CMakeCache.txt
RUN cmake -S . -G Ninja -B build -DCMAKE_BUILD_TYPE=Debug

ADD deployments/bun/create-hash-table.js /usr/src/bun/src/codegen/create-hash-table.js
ADD deployments/bun/bundle-functions.ts /usr/src/bun/src/codegen/bundle-functions-new.ts

RUN sed -i 's/import.meta.dir/__dirname/g' src/codegen/bundle-functions.ts
RUN sed -i 's/import.meta.path/__filename/g' src/codegen/bundle-functions.ts
RUN sed -i 's/Bun.env/process.env/g' src/codegen/replacements.ts

RUN yarn esbuild src/codegen/bundle-functions-new.ts --platform=node \
        --outfile=src/codegen/bundle-functions.js --format=cjs

# /usr/src/bun/src/codegen/create-hash-table.ts /usr/src/bun/src/bun.js/bindings/ZigGlobalObject.lut.txt /usr/src/bun/build/codegen/ZigGlobalObject.lut.h
# /usr/src/bun/src/codegen/bundle-functions.ts --debug=ON /usr/src/bun/build
# /usr/src/bun/src/codegen/create-hash-table.ts /usr/src/bun/src/bun.js/bindings/ProcessBindingNatives.cpp /usr/src/bun/build/codegen/ProcessBindingNatives.lut.h
# /usr/src/bun/src/codegen/create-hash-table.ts /usr/src/bun/src/bun.js/bindings/ProcessBindingConstants.cpp /usr/src/bun/build/codegen/ProcessBindingConstants.lut.h
# src/codegen/generate-jssink.ts /usr/src/bun/build/codegen
# /usr/src/bun/src/codegen/create-hash-table.ts /usr/src/bun/src/bun.js/bindings/BunObject.cpp /usr/src/bun/build/codegen/BunObject.lut.h
# /usr/src/bun/src/codegen/create-hash-table.ts /usr/src/bun/src/bun.js/bindings/JSBuffer.cpp /usr/src/bun/build/codegen/JSBuffer.lut.h
# /usr/src/bun/src/codegen/create-hash-table.ts /usr/src/bun/src/bun.js/bindings/BunProcess.cpp /usr/src/bun/build/codegen/BunProcess.lut.h
# /usr/src/bun/src/codegen/bundle-modules.ts --debug=ON /usr/src/bun/build
# /usr/src/bun/src/codegen/generate-classes.ts /usr/src/bun/src/bun.js/api/JSBundler.classes.ts /usr/src/bun/src/bun.js/api/bun.classes.ts /usr/src/bun/src/bun.js/api/crypto.classes.ts /usr/src/bun/src/bun.js/api/ffi.classes.ts /usr/src/bun/src/bun.js/api/filesystem_router.classes.ts /usr/src/bun/src/bun.js/api/html_rewriter.classes.ts /usr/src/bun/src/bun.js/api/server.classes.ts /usr/src/bun/src/bun.js/api/sockets.classes.ts /usr/src/bun/src/bun.js/node/node.classes.ts /usr/src/bun/src/bun.js/resolve_message.classes.ts /usr/src/bun/src/bun.js/test/jest.classes.ts /usr/src/bun/src/bun.js/webcore/encoding.classes.ts /usr/src/bun/src/bun.js/webcore/response.classes.ts /usr/src/bun/build/codegen

#RUN ninja -C build
