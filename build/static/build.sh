#!/bin/bash

echo ">> Build: ARM64 (GNU)"

GOOS=linux GOARCH=arm64 CGO_ENABLED=1 CC=aarch64-linux-gnu-gcc CXX=aarch64-linux-gnu-g++ \
    AR=aarch64-linux-gnu-ar go build -v \
    -ldflags="-extldflags=-static" -o "$(dirname "$0")/dragon.arm64" ./main.go

echo ">> Build: ARMHF (GNU)"

GOOS=linux GOARCH=arm CGO_ENABLED=1 CC=arm-linux-gnueabihf-gcc CXX=arm-linux-gnueabihf-g++ \
    AR=arm-linux-gnueabihf-ar go build -v \
    -ldflags="-extldflags=-static" -o "$(dirname "$0")/dragon.arm" ./main.go

echo ">> Build: AMD64 (GNU)"

GOOS=linux GOARCH=amd64 CGO_ENABLED=1 CC=x86_64-linux-gnu-gcc CXX=x86_64-linux-gnu-g++ \
    AR=x86_64-linux-gnu-ar go build -v \
    -ldflags="-extldflags=-static" -o "$(dirname "$0")/dragon.amd64" ./main.go

echo ">> Build: I386 (GNU)"

GOOS=linux GOARCH=386 CGO_ENABLED=1 CC=i686-linux-gnu-gcc CXX=i686-linux-gnu-g++ \
    AR=i686-linux-gnu-ar go build -v \
    -ldflags="-extldflags=-static" -o "$(dirname "$0")/dragon.i386" ./main.go

echo ">> Build: ARM64 (MUSL)"

GOOS=linux GOARCH=arm64 CGO_ENABLED=1 CC=musl-gcc go build -v \
    -ldflags="-extldflags=-static" -o "$(dirname "$0")/dragon.arm64.musl" ./main.go
