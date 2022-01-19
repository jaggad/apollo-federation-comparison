#!/bin/bash

function cleanup {
    kill "$ACCOUNTS_PID"
    kill "$PRODUCTS_PID"
    kill "$REVIEWS_PID"
}
trap cleanup EXIT

cargo build --bin accounts
cargo build --bin products
cargo build --bin reviews

cargo run --bin accounts &
ACCOUNTS_PID=$!

cargo run --bin products &
PRODUCTS_PID=$!

cargo run --bin reviews &
REVIEWS_PID=$!

sleep 1

rover supergraph compose --config ./src/router/supergraph.yaml > ./src/router/supergraph-schema.graphql

sleep 1

./src/router/router --config src/router/configuration.yaml --supergraph src/router/supergraph-schema.graphql