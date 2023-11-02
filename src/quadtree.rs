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

impl Quadtree {
    pub fn new(polygon: Polygon) -> Quadtree {
        Quadtree {
            polygon,
            data: Node::Leaf(Vec::new()),
            midpoint_status: PointStatus::Outside,
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
    pub fn add_polygon(&mut self, polygon: &Polygon) {
        match &mut self.data {
            Node::Internal(quadtrees) => {
                for quadtree in quadtrees.iter_mut() {
                    quadtree.add_polygon(polygon);
                }
            }
            Node::Leaf(arcs) => {
                arcs.extend(
                    polygon
                        .outline
                        .windows(2)
                        .map(|arc| Arc::new(&arc[0], &arc[1]))
                        .filter(|arc| {
                            self.polygon.contains_inside(arc.from())
                                || self.polygon.contains_inside(arc.to())
                                || !polygon.intersections(arc).is_empty()
                        }),
                );
                if arcs.len() > 100 {
                    self.split();
                }
            }
        }
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
        if self.polygon.contains_inside(point) {
            match &self.data {
                Node::Internal(quadtrees) => {
                    return quadtrees
                        .iter()
                        .find(|quadtree| quadtree.polygon.contains_inside(point))
                        .expect("should be inside as it is in parent")
                        .is_on_polygon(point)
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
