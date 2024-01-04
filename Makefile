DATA_DIR := tests/data
OSM_DIR := $(DATA_DIR)/osm
GEOJSON_DIR := $(DATA_DIR)/test_geojson
FMI_DIR := $(DATA_DIR)/fmi

INTERNET_OSM := https://cloud.p-fruck.de/s/pf9JfNabwDjrNL8/download/planet-coastlinespbf-cleaned.osm.pbf
NETWORK_OSM := $(OSM_DIR)/planet-coastlines.osm.pbf
PLANET := $(GEOJSON_DIR)/planet.geojson
NETWORK_GEOJSON := $(GEOJSON_DIR)/network.geojson

NETWORK_FMI := $(FMI_DIR)/network.fmi
NETWORK_CONTRACTED:= $(FMI_DIR)/network_contracted.json
NETWORK_HUBS:= $(FMI_DIR)/network_hubs.json
NETWORK_HUBS_PRUNED:= $(FMI_DIR)/network_hubs_pruned.json
NETWORK_TESTS := $(FMI_DIR)/network_tests.json

STGT_FMI := $(FMI_DIR)/stgtregbz.fmi
STGT_CONTRACTED:= $(FMI_DIR)/stgtregbz_contracted.json
STGT_HUBS:= $(FMI_DIR)/stgtregbz_hubs.json
STGT_HUBS_PRUNED:= $(FMI_DIR)/stgtregbz_hubs_pruned.json
STGT_TESTS_JSON := $(FMI_DIR)/stgtregbz_tests.json

NUM_TESTS := 1000
HOP_LIMIT := 2

dirs:
	mkdir tests/data/test_geojson/
	mkdir tests/data/image/
	mkdir tests/data/osm/
	mkdir tests/data/fmi/


download:
	curl $(INTERNET_OSM) -o $(NETWORK_OSM)

convert:
	cargo run --release --bin osm_geojson_converter -- --input $(NETWORK_OSM) --output  $(PLANET)

network:
	cargo run --release --bin preprocessor -- --input $(PLANET) --num-nodes 4000000 --output-network $(NETWORK_FMI) --output-geojson $(NETWORK_GEOJSON) --output-image tests/data/test_geojson/network.png

leaflet:
	docker run -dit --name leaflet -p 8080:80 -v ./public-html:/usr/local/apache2/htdocs/ httpd:2.4

server:
	cargo run --bin server --release  -- --fmi-path tests/data/fmi/network.fmi

test:
	cargo run --bin test --release -- --fmi-path tests/data/fmi/network.fmi --tests-path tests/data/fmi/tests.json --number-of-tests 1000


create_tests_stgt:
	cargo run --bin create_test --release -- --fmi-path $(STGT_FMI) --tests-path $(STGT_TESTS_JSON) --number-of-tests $(NUM_TESTS)

create_tests:
	cargo run --bin create_test --release -- --fmi-path $(NETWORK_FMI) --tests-path $(NETWORK_TESTS) --number-of-tests $(NUM_TESTS)


test_ch_stgt:
	cargo run --bin ch --release -- --fmi-path $(STGT_FMI) --contracted-graph $(STGT_CONTRACTED) --test-path $(STGT_TESTS_JSON)

test_ch:
	cargo run --bin ch --release -- --fmi-path $(NETWORK_FMI) --contracted-graph $(NETWORK_CONTRACTED) --test-path $(NETWORK_TESTS)


labels_stgt:
	cargo run --bin labels --release -- --contracted-graph $(STGT_CONTRACTED) --hub-graph $(STGT_HUBS) --hop-limit $(HOP_LIMIT)

labels:
	cargo run --bin labels --release -- --contracted-graph $(NETWORK_CONTRACTED) --hub-graph $(NETWORK_HUBS) --hop-limit $(HOP_LIMIT)


labels_test_stgt:
	cargo run --bin labels_test --release -- --hub-graph $(STGT_HUBS) --test-path $(STGT_TESTS_JSON)

labels_test:
	cargo run --bin labels_test --release -- --hub-graph $(NETWORK_HUBS) --test-path $(NETWORK_TESTS)


labels_prune_stgt:
	cargo run --bin labels_prune --release -- --hub-graph $(STGT_HUBS) --pruned-hub-graph $(STGT_HUBS_PRUNED) --test-path $(STGT_TESTS_JSON)

labels_prune:
	cargo run --bin labels_prune --release -- --hub-graph $(NETWORK_HUBS) --pruned_hub_graph $(NETWORK_HUBS_PRUNED) --test-path $(NETWORK_TESTS)


labels_size_stgt:
	cargo run --bin label_size --release -- --contracted-graph $(STGT_CONTRACTED) --test-path $(STGT_TESTS_JSON) --hop-limit $(HOP_LIMIT)

labels_size:
	cargo run --bin label_size --release -- --contracted-graph $(NETWORK_CONTRACTED) --test-path $(NETWORK_TESTS) --hop-limit $(HOP_LIMIT)

