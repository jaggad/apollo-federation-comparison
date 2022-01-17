#!/bin/bash

function cleanup {
    kill "$ACCOUNTS_PID"
    kill "$PRODUCTS_PID"
    kill "$REVIEWS_PID"
}
trap cleanup EXIT

go build -o /tmp/srv-accounts ./accounts
go build -o /tmp/srv-products ./products
go build -o /tmp/srv-reviews ./reviews

/tmp/srv-accounts &
ACCOUNTS_PID=$!

/tmp/srv-products &
PRODUCTS_PID=$!

/tmp/srv-reviews &
REVIEWS_PID=$!

sleep 1

rover supergraph compose --config router/supergraph.yaml > router/supergraph-schema.graphql
router/router --config router/configuration.yaml --supergraph router/supergraph-schema.graphql
