use crate::{Arc, Planet, Point, Polygon};

#[derive(Clone, Debug)]
pub struct SpatialPartition {
    pub boundary: Polygon,
    pub data: Node,
    pub midpoint_status: PointStatus,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PointStatus {
    Inside,
    Outside,
}

#[derive(Clone, Debug)]
pub enum Node {
    Internal(Vec<SpatialPartition>), // four children
    Leaf(Vec<Arc>),                  // a bucket of points
}

pub struct CollisionDetector {
    partition: SpatialPartition,
    planet: Planet,
}

const MAX_SIZE: usize = 10_000;

impl CollisionDetector {
    pub fn new(polygons: &Vec<Polygon>) -> CollisionDetector {
        CollisionDetector {
            partition: SpatialPartition::new_root(polygons),
            planet: Planet::new(),
        }
    }

    pub fn add_polygon(&mut self, polygon: Polygon) {
        polygon
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .for_each(|arc| {
                self.partition.add_arc(&arc);
            });

        self.planet.polygons.push(polygon);
    }

    pub fn update_midpoints(&mut self) {
        self.partition.update_midpoint(&self.planet);
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        self.partition.is_on_polygon(point)
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
            data: Node::Internal(
                polygons
                    .iter()
                    .cloned()
                    .map(|p| SpatialPartition::new_leaf(p))
                    .collect(),
            ),
            midpoint_status: PointStatus::Outside,
        }
    }

    pub fn new_leaf(polygon: Polygon) -> SpatialPartition {
        SpatialPartition {
            boundary: polygon,
            data: Node::Leaf(Vec::new()),
            midpoint_status: PointStatus::Outside,
        }
    }

    pub fn get_quadtrees(&self) -> Vec<SpatialPartition> {
        match &self.data {
            Node::Internal(q) => return q.iter().map(|q| q.get_quadtrees()).flatten().collect(),
            Node::Leaf(_) => vec![self.clone()],
        }
    }

    fn split(&mut self) {
        let mut arcs = Vec::new();
        if let Node::Leaf(old_arcs) = &self.data {
            arcs.extend(old_arcs);
        }
        self.data = Node::Internal(
            split_recangle(&self.boundary)
                .into_iter()
                .map(|rectangle| {
                    let mut q = SpatialPartition::new_leaf(rectangle);
                    set_midpoint(&mut q.boundary);

                    for arc in &arcs {
                        if q.boundary.intescts_or_inside(arc) {
                            q.add_arc(arc);
                        }
                    }
                    q
                })
                .collect(),
        );
    }

    fn add_arc(&mut self, arc: &Arc) {
        match &mut self.data {
            Node::Internal(quadtrees) => {
                for quadtree in quadtrees.iter_mut() {
                    if quadtree.boundary.intescts_or_inside(arc) {
                        quadtree.add_arc(arc);
                    }
                }
            }
            Node::Leaf(arcs) => {
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
        if planet.is_on_polygon(&self.boundary.inside_point) {
            self.midpoint_status = PointStatus::Inside;
        } else {
            self.midpoint_status = PointStatus::Outside;
        }

        if let Node::Internal(quadtrees) = &mut self.data {
            for quadtree in quadtrees {
                quadtree.update_midpoint(planet);
            }
        }
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        match &self.data {
            Node::Internal(quadtrees) => {
                let find = quadtrees
                    .iter()
                    .find(|quadtree| quadtree.boundary.contains_inside(point));
                if find.is_none() {
                    println!("error");
                    return false;
                }
                return find.unwrap().is_on_polygon(point);
            }
            Node::Leaf(arcs) => {
                let ray = Arc::new(point, &self.boundary.inside_point);
                let intersections = arcs.iter().filter_map(|arc| ray.intersection(arc)).count();
                match self.midpoint_status {
                    PointStatus::Inside => return intersections % 2 == 0,
                    PointStatus::Outside => return intersections % 2 == 1,
                }
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
