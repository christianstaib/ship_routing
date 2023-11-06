use std::sync::Mutex;

use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    Arc, CollisionDetection, ConvecQuadrilateral, Planet, Point, Polygon, SolidShape, Tiling,
};

#[derive(Clone)]
pub struct SpatialPartition {
    pub boundary: ConvecQuadrilateral,
    pub node_type: NodeType,
    pub max_size: usize,
    pub midpoint: Point,
    pub midpoint_flag: PointStatus,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PointStatus {
    Inside,
    Outside,
}

#[derive(Clone)]
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

pub struct PlanetGrid {
    pub spatial_partition: SpatialPartition,
    reference_point: Point,
    reference_point_status: PointStatus,
}

impl CollisionDetection for PlanetGrid {
    fn add_polygon(&mut self, polygon: &Polygon) {
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

        // TODO update midpoints
    }

    fn is_on_polygon(&self, point: &Point) -> bool {
        self.spatial_partition.is_on_polygon(point)
    }

    fn intersects_polygon(&self, _arc: &Arc) -> bool {
        todo!()
    }
}

impl PlanetGrid {
    pub fn new(max_size: usize) -> PlanetGrid {
        let polygons = Tiling::base_tiling();
        PlanetGrid {
            spatial_partition: SpatialPartition::new_root(&polygons, max_size),
            reference_point: Point::random(),
            reference_point_status: PointStatus::Outside,
        }
    }

    pub fn intersections(&self, ray: &Arc) -> Vec<Point> {
        let mut intersections = self.spatial_partition.intersections(ray);
        intersections.sort_by(|x, y| x.longitude().partial_cmp(&y.longitude()).unwrap());
        intersections.dedup(); // i dont know exactly why this is necesary, but it is :(
        intersections
    }

    pub fn update_midpoints(&mut self) {
        self.spatial_partition.propagte_status();
    }
}

impl SpatialPartition {
    pub fn new_root(polygons: &Vec<ConvecQuadrilateral>, max_size: usize) -> SpatialPartition {
        let boundary = ConvecQuadrilateral::new(&vec![
            Point::from_coordinate(0.0, 0.0),
            Point::from_coordinate(1.0, 1.0),
            Point::from_coordinate(1.0, -1.0),
            Point::from_coordinate(1.0, -1.0),
            Point::from_coordinate(0.0, 0.0),
        ]);
        let midpoint = boundary.get_midpoint();
        SpatialPartition {
            boundary,
            node_type: NodeType::Internal(
                polygons
                    .iter()
                    .cloned()
                    .map(|p| SpatialPartition::new_leaf(p, max_size))
                    .collect(),
            ),
            max_size,
            midpoint,
            midpoint_flag: PointStatus::Outside,
        }
    }

    pub fn new_leaf(boundary: ConvecQuadrilateral, max_size: usize) -> SpatialPartition {
        let midpoint = boundary.get_midpoint();
        SpatialPartition {
            boundary,
            node_type: NodeType::Leaf(Vec::with_capacity(max_size + 1)),
            max_size,
            midpoint,
            midpoint_flag: PointStatus::Outside,
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
                .map(|rectangle| SpatialPartition::new_leaf(rectangle, self.max_size))
                .collect(),
        );

        arcs.iter().for_each(|arc| self.add_arc(arc));
    }

    fn add_arc(&mut self, arc: &Arc) {
        let mut internals = vec![self];
        while let Some(parent) = internals.pop() {
            // needs to be done before the match block, as the match block pushes a mutable
            // reference to internals.
            if let NodeType::Leaf(arcs) = &mut parent.node_type {
                if arcs.len() >= parent.max_size {
                    parent.split();
                }
            }
            match &mut parent.node_type {
                NodeType::Internal(childs) => {
                    childs.sort_by(|x, y| {
                        Arc::new(&x.midpoint, arc.from())
                            .central_angle()
                            .total_cmp(&Arc::new(&y.midpoint, arc.from()).central_angle())
                    });
                    for child in childs.iter_mut() {
                        let contrains_from = child.boundary.contains(arc.from());
                        let contrains_to = child.boundary.contains(arc.to());
                        if contrains_from && contrains_to {
                            internals.push(child);
                            break;
                        } else if child.boundary.collides(arc) {
                            // expensive check
                            internals.push(child);
                        }
                    }
                }
                NodeType::Leaf(arcs) => {
                    arcs.push(arc.clone());
                }
            }
        }
    }

    pub fn propagte_status(&mut self) {
        let mut intersections = Vec::new();
        if let NodeType::Internal(quadtrees) = &self.node_type {
            for quadtree in quadtrees {
                let ray = Arc::new(&self.midpoint, &quadtree.midpoint);
                intersections.push(self.intersections(&ray).len());
            }
        }

        if let NodeType::Internal(quadtrees) = &mut self.node_type {
            for (quadtree, intersections) in quadtrees.iter_mut().zip(intersections) {
                if intersections % 2 == 0 {
                    quadtree.midpoint_flag = self.midpoint_flag;
                } else {
                    quadtree.midpoint_flag = self.midpoint_flag.other();
                }
            }
            quadtrees
                .iter_mut()
                .for_each(|quadtree| quadtree.propagte_status());
        }
    }

    pub fn get_leaf_polygons(&self) -> Vec<Arc> {
        match &self.node_type {
            NodeType::Internal(q) => q.iter().flat_map(|q| q.get_leaf_polygons()).collect(),
            NodeType::Leaf(_) => self
                .boundary
                .outline
                .windows(2)
                .map(|arc| Arc::new(&arc[0], &arc[1])._make_good_line())
                .flatten()
                .collect(),
        }
    }

    pub fn propagte_status_test(&self, planet: &Planet, out_planet: std::sync::Arc<Mutex<Planet>>) {
        if let NodeType::Internal(quadtrees) = &self.node_type {
            for quadtree in quadtrees {
                let ray = Arc::new(&self.midpoint, &quadtree.midpoint);
                let intersections = self.intersections(&ray);
                //assert_eq!(planet.intersections(&ray).len(), intersections.len());
                if intersections.len() % 2 == 0 {
                    if self.midpoint_flag != quadtree.midpoint_flag {
                        println!("error in propgation");
                        out_planet
                            .lock()
                            .unwrap()
                            .arcs
                            .extend(ray._make_good_line());
                        out_planet.lock().unwrap().points.extend(intersections);
                    }
                } else {
                    if self.midpoint_flag != quadtree.midpoint_flag.other() {
                        println!("error in propgation");
                        out_planet
                            .lock()
                            .unwrap()
                            .arcs
                            .extend(ray._make_good_line());
                        out_planet.lock().unwrap().points.extend(intersections);
                    }
                }
            }
            quadtrees
                .par_iter()
                .for_each(|quadtree| quadtree.propagte_status_test(planet, out_planet.clone()));
        }
    }

    pub fn intersections(&self, ray: &Arc) -> Vec<Point> {
        let mut intersections: Vec<Point> = match &self.node_type {
            NodeType::Internal(quadtrees) => quadtrees
                .iter()
                .filter(|quadtree| {
                    quadtree.boundary.contains(ray.from())
                        || quadtree.boundary.contains(ray.to())
                        || quadtree.boundary.collides(ray)
                })
                .map(|quadtree| quadtree.intersections(ray))
                .flatten()
                .collect(),
            NodeType::Leaf(arcs) => arcs
                .iter()
                .filter_map(|arc| ray.intersection(arc))
                //.filter(|p| self.boundary.contains(p))
                .collect(),
        };
        intersections.sort_by(|x, y| x.longitude().partial_cmp(&y.longitude()).unwrap());
        intersections.dedup(); // i dont know exactly why this is necesary, but it is :(
        intersections
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        let mut current = self;
        loop {
            match &current.node_type {
                NodeType::Internal(quadtrees) => {
                    current = quadtrees
                        .iter()
                        .find(|quadtree| quadtree.boundary.contains(point))
                        .unwrap()
                }
                NodeType::Leaf(arcs) => {
                    let ray = Arc::new(point, &self.midpoint);
                    let intersections = arcs.iter().filter_map(|arc| ray.intersection(arc)).count();
                    return (intersections % 2 == 0) == (self.midpoint_flag == PointStatus::Inside);
                }
            }
        }
    }
}
