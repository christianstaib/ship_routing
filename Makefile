GEOJSON_DIR := data/geojson
MBTILES_DIR := data/
DOCKER_IMG := metacollin/tippecanoe

generate_mbtiles:
	mkdir -p $(GEOJSON_DIR)
	mkdir -p $(MBTILES_DIR)
	rm -rf $(MBTILES_DIR)/*
	
	for file in $(GEOJSON_DIR)/*.geo.json; do \
    echo "generating mbtiles for $$file"; \
    IN_FILE=/mnt/data/$$file; \
    OUT_FILE=/mnt/data/$(MBTILES_DIR)/$$(basename $$file .geo.json).mbtiles; \
    docker run \
      --rm \
      -v $$(pwd):/mnt/data \
      $(DOCKER_IMG) \
      tippecanoe \
        --read-parallel \
        -zg \
        -o $$OUT_FILE \
        --drop-densest-as-needed \
        $$IN_FILE; \
  done

start_tileserver:
	docker run --rm -it -v ./$(MBTILES_DIR):/data -p 8080:8080 maptiler/tileserver-gl
