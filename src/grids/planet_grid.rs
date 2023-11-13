use std::sync::Mutex;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    meters_to_radians, Arc, Collides, CollisionDetection, Contains, ConvecQuadrilateral, Planet,
    Point, Polygon, Tiling,
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

impl CollisionDetection for SpatialPartition {
    fn add_polygon(&mut self, polygon: &Polygon) {
        self.add_polygon(polygon)
    }

    fn is_on_polygon(&self, point: &Point) -> bool {
        self.is_on_polygon(point)
    }

    fn intersects_polygon(&self, _arc: &Arc) -> bool {
        todo!()
    }
}

impl SpatialPartition {
    pub fn new_root(max_size: usize) -> SpatialPartition {
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
                Tiling::base_tiling()
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

    pub fn add_polygon(&mut self, polygon: &Polygon) {
        polygon
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .for_each(|arc| {
                self.add_arc(&arc);
            });

        if polygon.contains(&self.midpoint) {
            match self.midpoint_flag {
                PointStatus::Inside => self.midpoint_flag = PointStatus::Outside,
                PointStatus::Outside => self.midpoint_flag = PointStatus::Outside,
            }
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

    pub fn check_collision(&self, arc: &Arc) -> bool {
        let mut internals = vec![self];
        while let Some(parent) = internals.pop() {
            if let NodeType::Leaf(arcs) = &parent.node_type {
                if arcs.iter().any(|other| other.collides(arc)) {
                    return true;
                }
            } else if let NodeType::Internal(childs) = &parent.node_type {
                for child in childs.iter() {
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
        }
        false
    }

    fn add_arc(&mut self, arc: &Arc) {
        let mut internals = vec![self];
        while let Some(parent) = internals.pop() {
            // needs to be done before the match block, as the match block pushes a mutable
            // reference to internals.
            if let NodeType::Leaf(arcs) = &mut parent.node_type {
                arcs.push(arc.clone());
                if arcs.len() >= parent.max_size {
                    let outline =
                        Arc::new(&parent.boundary.outline[0], &parent.boundary.outline[1]);
                    if outline.central_angle() >= meters_to_radians(10.0) {
                        parent.split();
                    }
                }
            } else if let NodeType::Internal(childs) = &mut parent.node_type {
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
        }
    }

    pub fn propagate_status(&mut self) {
        let mut stack = Vec::new();
        stack.push(self);

        while let Some(current) = stack.pop() {
            let mut intersections = Vec::new();
            if let NodeType::Internal(quadtrees) = &current.node_type {
                for quadtree in quadtrees {
                    let ray = Arc::new(&current.midpoint, &quadtree.midpoint);
                    intersections.push(current.intersections(&ray).len());
                }
            }

            if let NodeType::Internal(quadtrees) = &mut current.node_type {
                for (quadtree, intersections) in quadtrees.iter_mut().zip(intersections) {
                    if intersections % 2 == 0 {
                        quadtree.midpoint_flag = current.midpoint_flag;
                    } else {
                        quadtree.midpoint_flag = current.midpoint_flag.other();
                    }
                    stack.push(quadtree);
                }
            }
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
                NodeType::Internal(childs) => {
                    for child in childs {
                        let contrains = child.boundary.contains(point);
                        if contrains {
                            current = &child;
                            break;
                        }
                    }
                }
                NodeType::Leaf(arcs) => {
                    let ray = Arc::new(point, &current.midpoint);
                    let intersections = arcs.iter().filter_map(|arc| ray.intersection(arc)).count();
                    return (intersections % 2 == 0)
                        == (current.midpoint_flag == PointStatus::Inside);
                }
            }
        }
    }
}
