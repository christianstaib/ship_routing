use crate::planet::Node;

pub fn coastline_to_feature(coastline: Vec<Node>) -> String {
    let begining = r#" { "type": "Feature", "geometry": { "type": "Polygon", "coordinates": [ [ "#;
    let formatted: Vec<String> = coastline
        .iter()
        .map(|&node| format!("[{}, {}]", node.lon, node.lat))
        .collect();
    let middle = formatted.join(",\n");
    let end = r#"] ] }, "properties": {} }"#;
    format!("{}\n{}\n{}", begining, middle, end)
}
