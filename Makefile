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

create_tests:
	cargo run --bin create_test --release -- --fmi-path tests/data/fmi/network.fmi --tests-path tests/data/fmi/network_tests.json --number-of-tests 1000

create_tests_stgt:
	cargo run --bin create_test --release -- --fmi-path tests/data/fmi/stgtregbz.fmi --tests-path tests/data/fmi/stgtregbz_tests.json --number-of-tests 1000


test_ch_stgt:
	cargo run --bin ch --release -- --fmi-path tests/data/fmi/stgtregbz.fmi --contracted-graph tests/data/fmi/stgtregbz_contracted.json --test-path tests/data/fmi/stgtregbz_tests.json

test_ch:
	cargo run --bin ch --release -- --fmi-path tests/data/fmi/network.fmi --contracted-graph tests/data/fmi/network_contracted.json --test-path tests/data/fmi/network_tests.json


test_labels_stgt:
	cargo run --bin labels --release -- --contracted-graph tests/data/fmi/stgtregbz_contracted.json --test-path tests/data/fmi/stgtregbz_tests.json

test_labels:
	cargo run --bin labels --release -- --contracted-graph tests/data/fmi/network_contracted.json --test-path tests/data/fmi/network_tests.json


test_better_labels:
	cargo run --bin better_labels --release -- --fmi-path tests/data/fmi/stgtregbz.fmi --test-path tests/data/fmi/stgtregbz_tests.json

test_labels_network:
	cargo run --bin labels --release -- --fmi-path tests/data/fmi/network.fmi --test-path tests/data/fmi/tests.json
