ARG RUST_VERSION=1.74.0
# first stage - build the executables
FROM rust:${RUST_VERSION}-slim-bookworm AS build

WORKDIR /app
RUN --mount=type=bind,source=src,target=/app/src \
    --mount=type=bind,source=Cargo.toml,target=/app/Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=/app/Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    mkdir /ship_routing \
    && for bin in preprocessor osm_geojson_converter server \
    ; do \
      cargo build --locked --release --bin $bin \
      && chmod +x ./target/release/$bin \
      && cp ./target/release/$bin /ship_routing/$bin \
    ; done

# second stage - generate network of nodes
FROM debian:bookworm-slim AS preprocess
WORKDIR /ship_routing
COPY --from=build /ship_routing /ship_routing
COPY filter.geojson ./
# retrieve coastline file and verify checksum
RUN apt-get update && apt-get install -y wget
RUN wget https://cloud.p-fruck.de/s/pf9JfNabwDjrNL8/download/planet-coastlinespbf-cleaned.osm.pbf -O /tmp/coastlines.pbf \
  && echo "e4f6df0f21b4273ebe07286e0995ef4afc9df7f11f061ffcafdf0ece4f2670f0 /tmp/coastlines.pbf" | sha256sum --check
RUN ./osm_geojson_converter -i /tmp/coastlines.pbf -o /tmp/planet.geojson
RUN ./preprocessor -i /tmp/planet.geojson -n 4000000 --output-network network.fmi --output-geojson /tmp/network.geojson --output-image /tmp/network.png

# final stage - run the webserver
FROM debian:bookworm-slim AS prod
WORKDIR /ship_routing
COPY --from=preprocess /ship_routing /ship_routing
COPY public-html/ public-html/

ENTRYPOINT ["./server"]
CMD ["-f", "network.fmi", "-b", "0.0.0.0:3030"]
EXPOSE 3030
