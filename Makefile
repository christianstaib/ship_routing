GEOJSON_DIR := data/geojson
MBTILES_DIR := data/mbtiles
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
	docker run --rm -it -v ./$(MBTILES_DIR):/data -p 8080:8080 maptiler/tileserver-gl --config data/data/config.json
