# Serve
FROM ubuntu:latest
RUN apt-get -qy update \
    && apt-get -qy install openssl \
    && rm -rf /var/lib/apt/lists/*

COPY . .
ENTRYPOINT ["./router", "--config", "./configuration.yaml", "--supergraph", "./supergraph-schema.graphql"]