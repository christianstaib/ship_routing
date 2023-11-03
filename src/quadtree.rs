use crate::{Arc, Planet, Point, Polygon};

#[derive(Clone, Debug)]
pub struct Quadtree {
    pub polygon: Polygon,
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
    Internal(Vec<Quadtree>), // four children
    Leaf(Vec<Arc>),          // a bucket of points
}

pub struct RootQuadtree {
    quadtrees: Vec<Quadtree>,
    planet: Planet,
}

const MAX_SIZE: usize = 10_000;

impl RootQuadtree {
    pub fn new(polygons: Vec<Polygon>) -> RootQuadtree {
        RootQuadtree {
            quadtrees: polygons
                .into_iter()
                .map(|mut polygon| {
                    set_midpoint(&mut polygon);
                    Quadtree::new(polygon)
                })
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

        self.planet.polygons.push(polygon);
    }

    pub fn update_midpoints(&mut self) {
        self.quadtrees
            .iter_mut()
            .for_each(|quadtree| quadtree.update_midpoint(&self.planet));
    }

    pub fn get_intersctions(&self, point: &Point) -> (usize, Point) {
        self.quadtrees
            .iter()
            .find_map(|quadtree| quadtree.get_intersctions(point))
            .unwrap()
    }

    pub fn is_on_polygon(&self, point: &Point, out_planet: &mut Planet) -> bool {
        self.quadtrees
            .iter()
            .map(|quadtree| quadtree.is_on_polygon(point, out_planet, &self.planet))
            .any(|x| x == true)
    }

    pub fn get_quadtrees(&self) -> Vec<Quadtree> {
        self.quadtrees
            .iter()
            .map(|q| q.get_quadtrees())
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

    pub fn get_quadtrees(&self) -> Vec<Quadtree> {
        match &self.data {
            Node::Internal(q) => return q.iter().map(|q| q.get_quadtrees()).flatten().collect(),
            Node::Leaf(_) => vec![self.clone()],
        }
    }

    pub fn get_intersctions(&self, point: &Point) -> Option<(usize, Point)> {
        if self.polygon.contains_inside(point) {
            match &self.data {
                Node::Internal(quadtrees) => {
                    let find = quadtrees
                        .iter()
                        .find(|quadtree| quadtree.polygon.contains_inside(point));
                    if find.is_none() {
                        panic!("error");
                    }
                    return find.unwrap().get_intersctions(point);
                }
                Node::Leaf(arcs) => {
                    let ray = Arc::new(point, &self.polygon.inside_point);
                    let intersections = arcs.iter().filter(|arc| ray.intersects(arc)).count();
                    return Some((intersections, self.polygon.inside_point.clone()));
                }
            }
        }
        None
    }

    fn split(&mut self) {
        let mut arcs = Vec::new();
        if let Node::Leaf(old_arcs) = &self.data {
            arcs.extend(old_arcs);
        }
        self.data = Node::Internal(
            split_recangle(&self.polygon)
                .into_iter()
                .map(|rectangle| {
                    let mut q = Quadtree::new(rectangle);
                    set_midpoint(&mut q.polygon);

                    for arc in &arcs {
                        if q.polygon.intescts_or_inside(arc) {
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
        if planet.is_on_polygon(&self.polygon.inside_point) {
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

    pub fn is_on_polygon(&self, point: &Point, out_planet: &mut Planet, planet: &Planet) -> bool {
        if self.polygon.contains_inside(point) {
            match &self.data {
                Node::Internal(quadtrees) => {
                    let find = quadtrees
                        .iter()
                        .find(|quadtree| quadtree.polygon.contains_inside(point));
                    if find.is_none() {
                        println!("error");
                        return false;
                    }
                    return find.unwrap().is_on_polygon(point, out_planet, planet);
                }
                Node::Leaf(arcs) => {
                    let ray = Arc::new(point, &self.polygon.inside_point);
                    let intersections: Vec<Point> = arcs
                        .iter()
                        .filter_map(|arc| ray.intersection(arc))
                        .collect();
                    match self.midpoint_status {
                        PointStatus::Inside => return intersections.len() % 2 == 0,
                        PointStatus::Outside => return intersections.len() % 2 == 1,
                    }
                }
            }
        }
        false
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
