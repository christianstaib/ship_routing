use std::sync::Arc;
use std::time::Instant;

use osm_test::geometry::Linestring;
use osm_test::geometry::Planet;
use osm_test::routing::get_route;
use osm_test::routing::Dijkstra;
use osm_test::routing::Graph;
use osm_test::spatial_graph::Fmi;
use serde_derive::{Deserialize, Serialize};
use warp::{http::Response, Filter};

use clap::Parser;

/// Starts a routing service on localhost:3030/route
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of .fmi file
    #[arg(short, long)]
    fmi_path: String,
}

#[derive(Deserialize, Serialize)]
struct RouteRequest {
    from: (f64, f64), // lon, lat
    to: (f64, f64),   // lon, lat
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let cors = warp::cors()
        .allow_any_origin() // For development. For production, specify allowed origins.
        .allow_headers(vec!["Content-Type"]) // Specify allowed headers
        .allow_methods(vec!["GET", "POST", "OPTIONS"]); // Specify allowed methods

    let graph = Graph::from_file(args.fmi_path.as_str());
    let graph = Arc::new(graph);
<<<<<<< HEAD
    let fmi = Arc::new(Fmi::from_file(args.fmi_path.as_str()));
=======
    let fmi = Arc::new(Fmi::new(args.fmi_path.as_str()));
>>>>>>> 2955f64335bf35c4052004516c0c1078874dcb11

    let promote = warp::post()
        .and(warp::path("route"))
        .and(warp::body::json())
        .map(move |route_request: RouteRequest| {
            let from = fmi.nearest(route_request.from.0, route_request.from.1);
            let to = fmi.nearest(route_request.to.0, route_request.to.1);

            let dijkstra = Dijkstra::new(&graph);
            let start = Instant::now();
            let (used_edges, cost) = dijkstra.dijkstra(from, to);
            let time = start.elapsed();
            let route = get_route(&graph, from, to, used_edges);

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

            println!(
                "route_request: {:>7} -> {:>7}, cost: {:>9}, took: {:>3}ms",
                from,
                to,
                cost,
                time.as_millis()
            );
            Response::builder().body(format!("{}", planet.to_geojson_str()))
        })
        .with(cors);

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}
