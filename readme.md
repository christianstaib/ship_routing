# Ship Routing

A submission for the "Algortihms on OSM-Data" lecture that enables users to perform ship routing, aka routing on sea.

## Usage

To build and run the application, the following commands can be used:

### Native

Requirements: `docker`, `cargo`, and at least 32gb of RAM.

- `make download`
- `make convert`: Convert the given PBF file into a planet geojson file by extracting and closing the coastlines.
- `make network`: Takes the planet geojson file and generates a given number of nodes located on water and creates a geojson file containing the generated nodes and a FMI file containing the graph representing the node neighbors.
- `make test`: Runs 1000 test routings
- `make leaflet` This command starts a local webserver that can be accessed on [localhost:3030](http://localhost:3030)
- `make server`: Starts routing backend
