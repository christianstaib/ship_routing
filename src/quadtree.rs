use crate::{Arc, ConvecQuadrilateral, Planet, Point, Polygon, SolidShape, Tiling};

#[derive(Clone, Debug)]
pub struct SpatialPartition {
    pub boundary: ConvecQuadrilateral,
    pub node_type: NodeType,
    pub midpoint: Point,
    pub midpoint_flag: PointStatus,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PointStatus {
    Inside,
    Outside,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Internal(Vec<SpatialPartition>), // four children
    Leaf(Vec<Arc>),                  // a bucket of points
}

impl PointStatus {
    pub fn other(&self) -> PointStatus {
        match self {
            PointStatus::Inside => PointStatus::Outside,
            PointStatus::Outside => PointStatus::Inside,
        }
    }
}

pub struct CollisionDetector {
    pub spatial_partition: SpatialPartition,
    planet: Planet,
    reference_point: Point,
    reference_point_status: PointStatus,
}

const MAX_SIZE: usize = 1_000;

impl CollisionDetector {
    pub fn new() -> CollisionDetector {
        let polygons = Tiling::base_tiling();
        CollisionDetector {
            spatial_partition: SpatialPartition::new_root(&polygons),
            planet: Planet::new(),
            reference_point: Point::random(),
            reference_point_status: PointStatus::Outside,
        }
    }

    pub fn add_polygon(&mut self, polygon: Polygon) {
        polygon
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .for_each(|arc| {
                self.spatial_partition.add_arc(&arc);
            });

        if polygon.contains(&self.reference_point) {
            match self.reference_point_status {
                PointStatus::Inside => self.reference_point_status = PointStatus::Outside,
                PointStatus::Outside => self.reference_point_status = PointStatus::Outside,
            }
        }

        self.planet.polygons.push(polygon);
    }

    pub fn intersections(&self, ray: &Arc) -> Vec<Point> {
        let mut intersections = self.spatial_partition.intersections(ray);
        intersections.dedup(); // if intersections of poylgon and arc on border of
        intersections
    }

    pub fn update_midpoints(&mut self) {
        // if self.planet.is_on_polygon(&self.spatial_partition.midpoint) {
        //     self.spatial_partition.midpoint_flag = PointStatus::Inside;
        // } else {
        //     self.spatial_partition.midpoint_flag = PointStatus::Outside;
        // }
        // self.spatial_partition.propagate_midpoint(&self.planet);
        self.spatial_partition.update_midpoint(&self.planet);
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        self.spatial_partition.is_on_polygon(point)
    }
}

impl SpatialPartition {
    pub fn new_root(polygons: &Vec<ConvecQuadrilateral>) -> SpatialPartition {
        let boundary = ConvecQuadrilateral::new(&vec![
            Point::from_geodetic(0.0, 0.0),
            Point::from_geodetic(1.0, 1.0),
            Point::from_geodetic(1.0, -1.0),
            Point::from_geodetic(0.0, 0.0),
        ]);
        let midpoint = boundary.get_midpoint();
        SpatialPartition {
            boundary,
            node_type: NodeType::Internal(
                polygons
                    .iter()
                    .cloned()
                    .map(|p| SpatialPartition::new_leaf(p))
                    .collect(),
            ),
            midpoint,
            midpoint_flag: PointStatus::Outside,
        }
    }

    pub fn new_leaf(boundary: ConvecQuadrilateral) -> SpatialPartition {
        let midpoint = boundary.get_midpoint();
        SpatialPartition {
            boundary,
            node_type: NodeType::Leaf(Vec::new()),
            midpoint,
            midpoint_flag: PointStatus::Outside,
        }
    }

    pub fn get_leafes(&self) -> Vec<SpatialPartition> {
        match &self.node_type {
            NodeType::Internal(q) => q.iter().flat_map(|q| q.get_leafes()).collect(),
            NodeType::Leaf(_) => vec![self.clone()],
        }
    }

    fn split(&mut self) {
        let mut arcs: Vec<Arc> = Vec::new();
        if let NodeType::Leaf(old_arcs) = &self.node_type {
            arcs.extend(old_arcs);
        }
        self.node_type = NodeType::Internal(
            self.boundary
                .split()
                .into_iter()
                .map(|rectangle| SpatialPartition::new_leaf(rectangle))
                .collect(),
        );

        arcs.iter().for_each(|arc| self.add_arc(arc));
    }

    fn add_arc(&mut self, arc: &Arc) {
        match &mut self.node_type {
            NodeType::Internal(quadtrees) => {
                for quadtree in quadtrees.iter_mut() {
                    let contrains_from = quadtree.boundary.contains(arc.from());
                    let contrains_to = quadtree.boundary.contains(arc.to());
                    if contrains_from && contrains_to {
                        quadtree.add_arc(arc);
                        // break;
                        // } else if contrains_from {
                        //     quadtree.add_arc(arc);
                        //     //  break;
                        // } else if contrains_to {
                        //     quadtree.add_arc(arc);
                        //     //  break;
                    } else if quadtree.boundary.intersects(arc) {
                        quadtree.add_arc(arc);
                    }
                }
            }
            NodeType::Leaf(arcs) => {
                arcs.push(arc.clone());

                if arcs.len() > MAX_SIZE {
                    self.split();
                }
            }
        }
    }

    pub fn update_midpoint(&mut self, planet: &Planet) {
        self.midpoint_flag = match planet.is_on_polygon(&self.midpoint) {
            true => PointStatus::Inside,
            false => PointStatus::Outside,
        };

        if let NodeType::Internal(quadtrees) = &mut self.node_type {
            quadtrees
                .iter_mut()
                .for_each(|quadtree| quadtree.update_midpoint(planet));
        }
    }

    pub fn intersections(&self, ray: &Arc) -> Vec<Point> {
        match &self.node_type {
            NodeType::Internal(quadtrees) => quadtrees
                .iter()
                // .filter(|quadtree| quadtree.boundary.intersects(ray))
                .map(|quadtree| quadtree.intersections(ray))
                .flatten()
                .collect(),
            NodeType::Leaf(arcs) => arcs
                .iter()
                .filter_map(|arc| ray.intersection(arc))
                //.filter(|p| self.boundary.contains(p))
                .collect(),
        }
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        match &self.node_type {
            NodeType::Internal(quadtrees) => {
                quadtrees
                    .iter()
                    .find(|quadtree| quadtree.boundary.contains(point))
                    .map_or_else(
                        || {
                            eprintln!("Error: Point is not within any quadtree boundary."); // Using eprintln! for errors
                            false
                        },
                        |quadtree| quadtree.is_on_polygon(point),
                    )
            }
            NodeType::Leaf(arcs) => {
                let ray = Arc::new(point, &self.midpoint);
                let intersections = arcs.iter().filter_map(|arc| ray.intersection(arc)).count();
                (intersections % 2 == 0) == (self.midpoint_flag == PointStatus::Inside)
            }
        }
    }
}
