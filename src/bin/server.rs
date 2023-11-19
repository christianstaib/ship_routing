use std::sync::Arc;
use std::time::Instant;

use osm_test::dijsktra::dijkstra;
use osm_test::geometry::Linestring;
use osm_test::geometry::Planet;
use osm_test::graph::get_route;
use osm_test::graph::Graph;
use osm_test::spatial_graph::Fmi;
use serde_derive::{Deserialize, Serialize};
use warp::{http::Response, Filter};

#[derive(Deserialize, Serialize)]
struct RouteRequest {
    from: (f64, f64), // lon, lat
    to: (f64, f64),   // lon, lat
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin() // For development. For production, specify allowed origins.
        .allow_headers(vec!["Content-Type"]) // Specify allowed headers
        .allow_methods(vec!["GET", "POST", "OPTIONS"]); // Specify allowed methods

    let graph = Arc::new(Graph::from_file("test_4M.fmi"));
    let fmi = Arc::new(Fmi::new("test_4M.fmi"));

    let promote = warp::post()
        .and(warp::path("route"))
        .and(warp::body::json())
        .map(move |route_request: RouteRequest| {
            let from = fmi.nearest(route_request.from.0, route_request.from.1);
            let to = fmi.nearest(route_request.to.0, route_request.to.1);

            let start = Instant::now();
            let (used_edges, cost) = dijkstra(&graph.clone(), from, to);
            let route = get_route(&graph, from, to, used_edges);
            let time = start.elapsed();

            let mut ids = Vec::new();
            if let Some(route) = route {
                ids.extend(route.edges.iter().map(|edge| edge.source_id));
                if let Some(last) = route.edges.last() {
                    ids.push(last.target_id);
                }
            }
            let path = fmi.convert_path(&ids);
            let linesstring = Linestring::new(path);
            let mut planet = Planet::new();
            planet.linestrings.push(linesstring);

            let response = planet.to_geojson_str();

            println!(
                "route_request: {:>7} -> {:>7}, cost: {:>9}, took: {:>3}ms",
                from,
                to,
                cost,
                time.as_millis()
            );
            Response::builder().body(format!("{}", response))
        })
        .with(cors);

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}
