GEOJSON_DIR := tests/data/test_geojson
MBTILES_DIR := tests/data/mbtiles
DOCKER_IMG := metacollin/tippecanoe

mbtiles:
	mkdir -p $(GEOJSON_DIR)
	mkdir -p $(MBTILES_DIR)
	
	for file in $(GEOJSON_DIR)/*.geojson; do \
    echo "generating mbtiles for $$file"; \
    IN_FILE=/mnt/data/$$file; \
    OUT_FILE=/mnt/data/$(MBTILES_DIR)/$$(basename $$file .geojson).mbtiles; \
    docker run \
      --rm \
      -v $$(pwd):/mnt/data \
      $(DOCKER_IMG) \
      tippecanoe \
        --read-parallel \
        -z10 \
        -o $$OUT_FILE \
        $$IN_FILE \
				--force; \
  done

start_tileserver:
	docker run --rm -it -v ./tests/data:/data -p 8080:8080 maptiler/tileserver-gl --config /data/config.json
	
merge:
	tile-join -o mbtiles/merged.mbtiles mbtiles/planet.mbtiles mbtiles/points.mbtiles --force

leaflet:
	docker run -dit --name leaflet -p 8080:80 -v ./public-html:/usr/local/apache2/htdocs/ httpd:2.4

network:
	cargo run --release --bin preprocessor -- --input tests/data/test_geojson/planet.geojson --num-nodes 4000000 --output-network tests/data/fmi/network.fmi --output-geojson tests/data/test_geojson/network.geojson --output-image tests/data/test_geojson/network.png

convert:
	cargo run --release --bin osm_geojson_converter -- --input tests/data/osm/planet-coastlines.osm.pbf --output tests/data/test_geojson/planet.geojson

download:
	curl https://cloud.p-fruck.de/s/pf9JfNabwDjrNL8/download/planet-coastlinespbf-cleaned.osm.pbf -o tests/data/osm/planet-coastlines.osm.pbf

server:
	cargo run --bin server --release  -- --fmi-path tests/data/fmi/network.fmi
	
