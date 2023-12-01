# Ship Routing

A submission for the "Algortihms on OSM-Data" lecture that enables users to perform ship routing, aka routing on sea.

## Usage

To build and run the application, the following commands can be used:

- `make convert`: Convert the given PBF file into a planet geojson file by extracting and closing the coastlines.
- `make network`: Takes the planet geojson file and generates a given number of nodes located on water and creates a geojson file containing the generated nodes and a FMI file containing the graph representing the node neighbors.
