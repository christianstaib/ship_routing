GEOJSON_DIR := tests/data/test_geojson
MBTILES_DIR := tests/data/mbtiles
DOCKER_IMG := metacollin/tippecanoe

generate_mbtiles:
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
        -zg \
        -o $$OUT_FILE \
        --drop-densest-as-needed \
        $$IN_FILE \
				--force; \
  done

start_tileserver:
	docker run --rm -it -v ./tests/data:/data -p 8080:8080 maptiler/tileserver-gl --config /data/config.json
	
merge:
	tile-join -o mbtiles/merged.mbtiles mbtiles/planet.mbtiles mbtiles/points.mbtiles --force

leaflet:
	docker run -dit --name leaflet -p 8080:80 -v ./public-html:/usr/local/apache2/htdocs/ httpd:2.4
