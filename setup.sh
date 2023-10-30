#!/bin/bash

cargo install diesel_cli --no-default-features --features postgres

docker run \
    -d \
    -it \
    --rm \
    -p 5432:5432 \
    -e POSTGRES_USER=dev \
    -e POSTGRES_PASSWORD=dev \
    -e POSTGRES_DB=dev \
    postgres:alpine
