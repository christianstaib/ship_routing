use crate::{Arc, Planet, Point, Polygon, SolidShape};

#[derive(Clone, Debug)]
pub struct SpatialPartition {
    pub boundary: Polygon,
    pub node_type: NodeType,
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

pub struct CollisionDetector {
    spatial_partition: SpatialPartition,
    planet: Planet,
}

const MAX_SIZE: usize = 25_000;

impl CollisionDetector {
    pub fn new(polygons: &Vec<Polygon>) -> CollisionDetector {
        CollisionDetector {
            spatial_partition: SpatialPartition::new_root(polygons),
            planet: Planet::new(),
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

        self.planet.polygons.push(polygon);
    }

    pub fn update_midpoints(&mut self) {
        self.spatial_partition.update_midpoint(&self.planet);
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        self.spatial_partition.is_on_polygon(point)
    }
}

impl SpatialPartition {
    pub fn new_root(polygons: &Vec<Polygon>) -> SpatialPartition {
        let polgon = Polygon::new(vec![
            Point::from_geodetic(0.0, 0.0),
            Point::from_geodetic(1.0, 1.0),
            Point::from_geodetic(1.0, -1.0),
            Point::from_geodetic(0.0, 0.0),
        ]);
        SpatialPartition {
            boundary: polgon,
            node_type: NodeType::Internal(
                polygons
                    .iter()
                    .cloned()
                    .map(|p| SpatialPartition::new_leaf(p))
                    .collect(),
            ),
            midpoint_flag: PointStatus::Outside,
        }
    }

    pub fn new_leaf(polygon: Polygon) -> SpatialPartition {
        SpatialPartition {
            boundary: polygon,
            node_type: NodeType::Leaf(Vec::new()),
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
            split_recangle(&self.boundary)
                .into_iter()
                .map(|rectangle| {
                    let mut q = SpatialPartition::new_leaf(rectangle);
                    set_midpoint(&mut q.boundary);

                    q
                })
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
                        break;
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

    pub fn add_polygon(&mut self, polygon: &Polygon) {
        polygon
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .for_each(|arc| self.add_arc(&arc));
    }

    pub fn update_midpoint(&mut self, planet: &Planet) {
        self.midpoint_flag = match planet.is_on_polygon(&self.boundary.inside_point) {
            true => PointStatus::Inside,
            false => PointStatus::Outside,
        };

        if let NodeType::Internal(quadtrees) = &mut self.node_type {
            quadtrees
                .iter_mut()
                .for_each(|quadtree| quadtree.update_midpoint(planet));
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
                let ray = Arc::new(point, &self.boundary.inside_point);
                let intersections = arcs.iter().filter_map(|arc| ray.intersection(arc)).count();
                (intersections % 2 == 0) == (self.midpoint_flag == PointStatus::Inside)
            }
        }
    }
}

fn set_midpoint(rectangle: &mut Polygon) {
    let d0 = Arc::new(&rectangle.outline[0], &rectangle.outline[2]);
    let d1 = Arc::new(&rectangle.outline[1], &rectangle.outline[3]);
    rectangle.inside_point = d0.intersection(&d1).unwrap();
}

fn split_recangle(rectangle: &Polygon) -> Vec<Polygon> {
    assert!(rectangle.outline.len() == 5);
    let arcs: Vec<Arc> = rectangle
        .outline
        .windows(2)
        .map(|arc| Arc::new(&arc[0], &arc[1]))
        .collect();

    let o = rectangle.outline.clone();

    let m: Vec<Point> = arcs.iter().map(|arc| arc.middle()).collect();

    let d0 = Arc::new(&m[0], &m[2]);
    let d1 = Arc::new(&m[1], &m[3]);

    let middle = d0.intersection(&d1).expect("should intersection");
    let mut subs = Vec::new();
    let mut p0 = Polygon::new(vec![
        m[3].clone(),
        middle.clone(),
        m[2].clone(),
        o[3].clone(),
        m[3].clone(),
    ]);
    set_midpoint(&mut p0);
    subs.push(p0);

    let mut p1 = Polygon::new(vec![
        middle.clone(),
        m[1].clone(),
        o[2].clone(),
        m[2].clone(),
        middle.clone(),
    ]);
    set_midpoint(&mut p1);
    subs.push(p1);

    let mut p2 = Polygon::new(vec![
        m[0].clone(),
        o[1].clone(),
        m[1].clone(),
        middle.clone(),
        m[0].clone(),
    ]);
    set_midpoint(&mut p2);
    subs.push(p2);

    let mut p3 = Polygon::new(vec![
        o[0].clone(),
        m[0].clone(),
        middle.clone(),
        m[3].clone(),
        o[0].clone(),
    ]);
    set_midpoint(&mut p3);
    subs.push(p3);

    subs
}
