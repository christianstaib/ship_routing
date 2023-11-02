use crate::{Arc, Planet, Point, Polygon};

#[derive(Clone, Debug)]
pub struct Quadtree {
    polygon: Polygon,
    data: Node,
    midpoint_status: PointStatus,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PointStatus {
    Inside,
    Outside,
}

#[derive(Clone, Debug)]
enum Node {
    Internal(Vec<Quadtree>), // four children
    Leaf(Vec<Arc>),          // a bucket of points
}

pub struct RootQuadtree {
    quadtrees: Vec<Quadtree>,
    planet: Planet,
}

const MAX_SIZE: usize = 100;

impl RootQuadtree {
    pub fn new(polygons: Vec<Polygon>) -> RootQuadtree {
        RootQuadtree {
            quadtrees: polygons
                .into_iter()
                .map(|polygon| Quadtree::new(polygon))
                .collect(),
            planet: Planet::new(),
        }
    }

    pub fn add_polygon(&mut self, polygon: Polygon) {
        polygon
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .for_each(|arc| {
                self.quadtrees.iter_mut().for_each(|quadtree| {
                    if quadtree.polygon.intescts_or_inside(&arc) {
                        quadtree.add_arc(&arc);
                    }
                })
            });
    }

    pub fn update_midpoints(&mut self) {
        self.quadtrees
            .iter_mut()
            .for_each(|quadtree| quadtree.update_midpoint(&self.planet));
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        self.quadtrees
            .iter()
            .map(|quadtree| quadtree.is_on_polygon(point))
            .any(|x| x == true)
    }

    pub fn get_polygons(&self) -> Vec<Polygon> {
        self.quadtrees
            .iter()
            .map(|q| q.get_polygons())
            .flatten()
            .collect()
    }
}

impl Quadtree {
    pub fn new(polygon: Polygon) -> Quadtree {
        Quadtree {
            polygon,
            data: Node::Leaf(Vec::new()),
            midpoint_status: PointStatus::Outside,
        }
    }

    pub fn get_polygons(&self) -> Vec<Polygon> {
        match &self.data {
            Node::Internal(q) => return q.iter().map(|q| q.get_polygons()).flatten().collect(),
            Node::Leaf(_) => vec![self.polygon.clone()],
        }
    }

    fn split(&mut self) {
        self.data = Node::Internal(
            split_recangle(&self.polygon)
                .into_iter()
                .map(|rectangle| Quadtree::new(rectangle))
                .collect(),
        );
    }

    fn add_arc(&mut self, arc: &Arc) {
        match &mut self.data {
            Node::Internal(quadtrees) => {
                for quadtree in quadtrees.iter_mut() {
                    if quadtree.polygon.intescts_or_inside(arc) {
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
        match &mut self.data {
            Node::Internal(quadtrees) => {
                for quadtree in quadtrees {
                    quadtree.update_midpoint(planet);
                }
            }
            Node::Leaf(_) => {
                if planet.is_on_polygon(&self.polygon.inside_point) {
                    self.midpoint_status = PointStatus::Inside;
                } else {
                    self.midpoint_status = PointStatus::Outside;
                }
            }
        }
    }

    pub fn is_on_polygon(&self, point: &Point) -> bool {
        println!("hier. midpoint is {:?}", self.polygon.inside_point);
        if self.polygon.contains_inside(point) {
            match &self.data {
                Node::Internal(quadtrees) => {
                    let find = quadtrees
                        .iter()
                        .find(|quadtree| quadtree.polygon.contains_inside(point));
                    if find.is_none() {
                        return false;
                    }
                    return find.unwrap().is_on_polygon(point);
                }
                Node::Leaf(arcs) => {
                    let ray = Arc::new(point, &self.polygon.inside_point);
                    let intersections = arcs.iter().filter(|arc| ray.intersects(arc)).count();
                    match self.midpoint_status {
                        PointStatus::Inside => return intersections % 2 == 0,
                        PointStatus::Outside => return intersections % 2 == 1,
                    }
                }
            }
        }
        false
    }
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
    let p0 = Polygon::new(vec![
        m[3].clone(),
        middle.clone(),
        m[2].clone(),
        o[3].clone(),
        m[3].clone(),
    ]);

    let p1 = Polygon::new(vec![
        middle.clone(),
        m[1].clone(),
        o[2].clone(),
        m[2].clone(),
        middle.clone(),
    ]);

    let p2 = Polygon::new(vec![
        m[0].clone(),
        o[1].clone(),
        m[1].clone(),
        middle.clone(),
        m[0].clone(),
    ]);

    let p3 = Polygon::new(vec![
        o[0].clone(),
        m[0].clone(),
        middle.clone(),
        m[3].clone(),
        o[0].clone(),
    ]);
    vec![p0, p1, p2, p3]
}
