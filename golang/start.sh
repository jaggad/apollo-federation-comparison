#!/bin/bash

# $GOROOT="/Users/jack.edgson/Development/federation-comparison/golang"

function cleanup {
    kill "$ACCOUNTS_PID"
    kill "$PRODUCTS_PID"
    kill "$REVIEWS_PID"
}
trap cleanup EXIT

# $GOPATH="/Users/jack.edgson/Development/federation-comparison/golang"


# set GOPATH=/Users/jack.edgson/Development/federation-comparison/golang


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
