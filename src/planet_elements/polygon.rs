use std::{collections::HashMap, error::Error, f64::consts::PI};

use geojson::{Feature, Geometry, Value};

use crate::Planet;

use super::{Arc, Point};

#[derive(Clone, Debug)]
pub struct Polygon {
    pub outline: Vec<Point>,
    inside_point: Point,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum PointClassification {
    Visited(Point),
    Unvisited(Point),
    InIntersection(Point),
    OutIntersection(Point),
}

impl PointClassification {
    pub fn inner(&self) -> &Point {
        match self {
            PointClassification::Visited(p) => p,
            PointClassification::Unvisited(p) => p,
            PointClassification::InIntersection(p) => p,
            PointClassification::OutIntersection(p) => p,
        }
    }
}

impl Polygon {
    pub fn new(outline: Vec<Point>) -> Polygon {
        let mut polygon = Polygon {
            outline,
            inside_point: Point::random(),
        };

        polygon.inside_point = polygon.get_inside_point();
        polygon
    }

    pub fn from_vec(vec: Vec<Vec<f64>>) -> Result<Polygon, Box<dyn Error>> {
        let outline = vec
            .into_iter()
            .map(|point| Point::from_vec(point).unwrap())
            .collect();
        Ok(Polygon::new(outline))
    }

    pub fn get_inside_point(&self) -> Point {
        let ab = Arc::new(&self.outline[0], &self.outline[1]);
        let middle = ab.middle();
        let destination =
            Point::destination_point(&middle, ab.initial_bearing() + (PI / 2.0), -0.01);
        let md = Arc::new(&middle, &destination);
        let mut intersections = self.intersections(&md);
        intersections.sort_by(|&a, &b| {
            let a_dist = Arc::new(&middle, &a).central_angle();
            let b_dist = Arc::new(&middle, &b).central_angle();
            a_dist.partial_cmp(&b_dist).unwrap()
        });

        // make sure middle is in list
        if let Some(first) = intersections.first() {
            if !middle.equals(first) {
                intersections.insert(0, middle);
            }
        } else {
            intersections.insert(0, middle);
        }

        intersections.push(destination);

        Arc::new(&intersections[0], &intersections[1]).middle()
    }

    pub fn contains_inside(&self, point: &Point) -> bool {
        let ray = Arc::new(point, &self.inside_point);
        let intersections = self.intersections(&ray).len();

        intersections % 2 == 0
    }

    // (self_arc, other_arc, intersection)
    pub fn intersections_polygon(&self, other: &Polygon) -> Vec<(Arc, Arc, PointClassification)> {
        let mut intersections = Vec::new();
        self.outline
            .windows(2)
            .map(|self_arc| Arc::new(&self_arc[0], &self_arc[1]))
            .for_each(|self_arc| {
                other
                    .outline
                    .windows(2)
                    .map(|other_arc| Arc::new(&other_arc[0], &other_arc[1]))
                    .for_each(|other_arc| {
                        if let Some(intersection) = self_arc.intersection(&other_arc) {
                            let intersection =
                                match self_arc.normal().dot(&other_arc.to().vec()) >= 0.0 {
                                    true => PointClassification::InIntersection(intersection),
                                    false => PointClassification::OutIntersection(intersection),
                                };
                            intersections.push((self_arc, other_arc, intersection));
                        }
                    });
            });
        intersections
    }

    // clip self by other
    pub fn clip(&self, other: &Polygon) -> Vec<Polygon> {
        let intersections = self.intersections_polygon(other);

        if intersections.is_empty() {
            if other.contains_inside(&self.outline[0]) {
                return vec![self.clone()];
            } else {
                return Vec::new();
            }
        }

        let mut self_map: Vec<(Arc, Vec<PointClassification>)> = Vec::new();
        let mut other_map: Vec<(Arc, Vec<PointClassification>)> = Vec::new();

        for (self_arc, other_arc, point_class) in intersections.iter() {
            if let Some((_, classes)) = self_map.iter_mut().find(|(a, _)| *a == *self_arc) {
                classes.push(point_class.clone());
            } else {
                self_map.push((self_arc.clone(), vec![point_class.clone()]));
            }
            if let Some((_, classes)) = other_map.iter_mut().find(|(a, _)| *a == *other_arc) {
                classes.push(point_class.clone());
            } else {
                other_map.push((other_arc.clone(), vec![point_class.clone()]));
            }
        }

        self_map.iter_mut().for_each(|(arc, classes)| {
            classes.sort_by(|a, b| {
                let a = a.inner();
                let b = b.inner();
                let a = Arc::new(arc.from(), a).central_angle();
                let b = Arc::new(arc.from(), b).central_angle();
                a.partial_cmp(&b).unwrap()
            })
        });
        other_map.iter_mut().for_each(|(arc, classes)| {
            classes.sort_by(|a, b| {
                let a = a.inner();
                let b = b.inner();
                let a = Arc::new(arc.from(), a).central_angle();
                let b = Arc::new(arc.from(), b).central_angle();
                a.partial_cmp(&b).unwrap()
            })
        });

        let mut self_orderd: Vec<PointClassification> = self
            .outline
            .iter()
            .map(|p| PointClassification::Unvisited(p.clone()))
            .collect();
        for i in (0..(self_orderd.len() - 2)).rev() {
            let pi = self_orderd[i];
            let pi1 = self_orderd[i + 1];
            let arc = Arc::new(&pi.inner(), &pi1.inner());
            if let Some((_, classes)) = self_map.iter_mut().find(|(a, _)| *a == arc) {
                classes
                    .iter()
                    .rev()
                    .for_each(|c| self_orderd.insert(i + 1, *c));
            }
        }
        let mut other_orderd: Vec<PointClassification> = other
            .outline
            .iter()
            .map(|p| PointClassification::Unvisited(p.clone()))
            .collect();
        for i in (0..(other_orderd.len() - 1)).rev() {
            let pi = other_orderd[i];
            let pi1 = other_orderd[i + 1];
            let arc = Arc::new(&pi.inner(), &pi1.inner());
            if let Some((_, classes)) = other_map.iter_mut().find(|(a, _)| *a == arc) {
                classes
                    .iter()
                    .rev()
                    .for_each(|c| other_orderd.insert(i + 1, *c));
            }
        }

        // planet.lines.extend(
        //     other_orderd
        //         .windows(2)
        //         .map(|x| Arc::new(&x[0].inner(), &x[1].inner())),
        // );

        let mut polygons = Vec::new();

        while let Some(mut idx) = self_orderd.iter().position(|class| match class {
            PointClassification::InIntersection(_) => true,
            _ => false,
        }) {
            let mut outline = Vec::new();

            let incoming = self_orderd[idx].inner().clone();
            outline.push(incoming);
            self_orderd[idx] = PointClassification::Visited(incoming);
            'self_loop: loop {
                idx = (idx - 1) % self_orderd.len();
                match self_orderd[idx] {
                    PointClassification::Visited(_) => panic!("should not happen"),
                    PointClassification::Unvisited(p) => {
                        outline.push(p);
                        self_orderd[idx] = PointClassification::Visited(p);
                    }
                    PointClassification::InIntersection(_) => panic!("should not happen"),
                    PointClassification::OutIntersection(p) => {
                        self_orderd[idx] = PointClassification::Visited(p);
                        outline.push(p);
                        break 'self_loop;
                    }
                }
            }
            idx = other_orderd
                .iter()
                .position(|class| class.inner() == outline.last().unwrap())
                .unwrap();
            let incoming = other_orderd[idx].inner().clone();
            outline.push(incoming);
            other_orderd[idx] = PointClassification::Visited(incoming);
            'other_loop: loop {
                idx = (idx - 1) % other_orderd.len();
                match other_orderd[idx] {
                    PointClassification::Visited(_) => panic!("visited should not happen"),
                    PointClassification::Unvisited(p) => {
                        outline.push(p);
                        other_orderd[idx] = PointClassification::Visited(p);
                    }
                    PointClassification::OutIntersection(_) => panic!("out should not happen"),
                    PointClassification::InIntersection(p) => {
                        other_orderd[idx] = PointClassification::Visited(p);
                        outline.push(p);
                        break 'other_loop;
                    }
                }
            }

            polygons.push(Polygon::new(outline));
        }

        polygons
    }

    pub fn intersections(&self, line: &Arc) -> Vec<Point> {
        let intersection: Vec<Point> = self
            .outline
            .windows(2)
            .filter_map(|outline| {
                let outline = Arc::new(&outline[0], &outline[1]);
                line.intersection(&outline)
            })
            .collect();
        //intersection.dedup();
        intersection
    }

    pub fn to_feature(&self) -> Feature {
        let polygon = self
            .outline
            .iter()
            .map(|&coordinate| vec![coordinate.lon(), coordinate.lat()])
            .collect();

        let polygon = Geometry::new(Value::Polygon(vec![polygon]));
        Feature {
            bbox: None,
            geometry: Some(polygon),
            id: None,
            properties: None,
            foreign_members: None,
        }
    }

    pub fn to_json(&self) -> String {
        self.to_feature().to_string()
    }
}
