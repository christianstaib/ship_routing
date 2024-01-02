DATA_DIR := tests/data
OSM_DIR := $(DATA_DIR)/osm
GEOJSON_DIR := $(DATA_DIR)/test_geojson
FMI_DIR := $(DATA_DIR)/fmi

NETWORK_FMI := $(FMI_DIR)/network.fmi
NETWORK_CONTRACTED:= $(FMI_DIR)/network_contracted.json
NETWORK_HUBS:= $(FMI_DIR)/network_hubs.json
NETWORK_TESTS := $(FMI_DIR)/network_tests.json

STGT_FMI := $(FMI_DIR)/stgtregbz.fmi
STGT_CONTRACTED:= $(FMI_DIR)/stgtregbz_contracted.json
STGT_HUBS:= $(FMI_DIR)/stgtregbz_hubs.json
STGT_TESTS_JSON := $(FMI_DIR)/stgtregbz_tests.json

NUM_TESTS := 1000

dirs:
	mkdir tests/data/test_geojson/
	mkdir tests/data/image/
	mkdir tests/data/osm/
	mkdir tests/data/fmi/


download:
	curl https://cloud.p-fruck.de/s/pf9JfNabwDjrNL8/download/planet-coastlinespbf-cleaned.osm.pbf -o tests/data/osm/planet-coastlines.osm.pbf

convert:
	cargo run --release --bin osm_geojson_converter -- --input tests/data/osm/planet-coastlines.osm.pbf --output tests/data/test_geojson/planet.geojson

network:
	cargo run --release --bin preprocessor -- --input tests/data/test_geojson/planet.geojson --num-nodes 4000000 --output-network tests/data/fmi/network.fmi --output-geojson tests/data/test_geojson/network.geojson --output-image tests/data/test_geojson/network.png

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


test_labels_stgt:
	cargo run --bin labels --release -- --contracted-graph $(STGT_CONTRACTED) --hub-graph $(STGT_HUBS) --test-path $(STGT_TESTS_JSON)

test_labels:
	cargo run --bin labels --release -- --contracted-graph $(NETWORK_CONTRACTED) --hub-graph $(NETWORK_HUBS) --test-path $(NETWORK_TESTS)

