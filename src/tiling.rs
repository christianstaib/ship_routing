use crate::{Arc, Point, SolidShape};

pub struct ConvecQuadrilateral {
    pub outline: Vec<Point>,
}

impl SolidShape for ConvecQuadrilateral {
    fn contains(&self, point: &crate::Point) -> bool {
        self.outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .all(|arc| arc.is_on_righthand_side(point))
    }

    fn intersects(&self, arc: &Arc) -> bool {
        !self.intersections(arc).is_empty()
    }
}

pub trait Splitable: SolidShape {
    fn split(&self) -> Vec<Box<dyn Splitable>>;
}

impl Splitable for ConvecQuadrilateral {
    fn split(&self) -> Vec<Box<dyn Splitable>> {
        let arcs: Vec<Arc> = self
            .outline
            .windows(2)
            .map(|arc| Arc::new(&arc[0], &arc[1]))
            .collect();

        let o = self.outline.clone();

        let m: Vec<Point> = arcs.iter().map(|arc| arc.middle()).collect();

        let d0 = Arc::new(&m[0], &m[2]);
        let d1 = Arc::new(&m[1], &m[3]);

        let middle = d0.intersection(&d1).expect("should intersection");

        let mut subs: Vec<Box<dyn Splitable>> = Vec::new();
        let p0 = ConvecQuadrilateral::new(&vec![
            m[3].clone(),
            middle.clone(),
            m[2].clone(),
            o[3].clone(),
            m[3].clone(),
        ]);
        subs.push(Box::new(p0));

        let p1 = ConvecQuadrilateral::new(&vec![
            middle.clone(),
            m[1].clone(),
            o[2].clone(),
            m[2].clone(),
            middle.clone(),
        ]);
        subs.push(Box::new(p1));

        let p2 = ConvecQuadrilateral::new(&vec![
            m[0].clone(),
            o[1].clone(),
            m[1].clone(),
            middle.clone(),
            m[0].clone(),
        ]);
        subs.push(Box::new(p2));

        let p3 = ConvecQuadrilateral::new(&vec![
            o[0].clone(),
            m[0].clone(),
            middle.clone(),
            m[3].clone(),
            o[0].clone(),
        ]);
        subs.push(Box::new(p3));

        subs
    }
}

impl ConvecQuadrilateral {
    pub fn new(outline: &Vec<Point>) -> ConvecQuadrilateral {
        ConvecQuadrilateral {
            outline: outline.clone(),
        }
    }

    pub fn intersections(&self, line: &Arc) -> Vec<Point> {
        self.outline
            .windows(2)
            .filter_map(|outline| {
                let outline = Arc::new(&outline[0], &outline[1]);
                line.intersection(&outline)
            })
            .collect()
    }
}
