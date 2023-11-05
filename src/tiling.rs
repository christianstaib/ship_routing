use crate::{Arc, Point, SolidShape};

#[derive(Clone)]
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

impl ConvecQuadrilateral {
    pub fn split(&self) -> Vec<ConvecQuadrilateral> {
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

        let mut subs = Vec::new();
        let p0 = ConvecQuadrilateral::new(&vec![
            m[3].clone(),
            middle.clone(),
            m[2].clone(),
            o[3].clone(),
            m[3].clone(),
        ]);
        subs.push(p0);

        let p1 = ConvecQuadrilateral::new(&vec![
            middle.clone(),
            m[1].clone(),
            o[2].clone(),
            m[2].clone(),
            middle.clone(),
        ]);
        subs.push(p1);

        let p2 = ConvecQuadrilateral::new(&vec![
            m[0].clone(),
            o[1].clone(),
            m[1].clone(),
            middle.clone(),
            m[0].clone(),
        ]);
        subs.push(p2);

        let p3 = ConvecQuadrilateral::new(&vec![
            o[0].clone(),
            m[0].clone(),
            middle.clone(),
            m[3].clone(),
            o[0].clone(),
        ]);
        subs.push(p3);

        subs
    }
}

impl ConvecQuadrilateral {
    pub fn new(outline: &Vec<Point>) -> ConvecQuadrilateral {
        ConvecQuadrilateral {
            outline: outline.clone(),
        }
    }

    pub fn collides(&self, arc: &Arc) -> bool {
        self.outline.windows(2).any(|outline| {
            let outline = Arc::new(&outline[0], &outline[1]);
            arc.collides_arc(&outline)
        })
    }

    pub fn get_midpoint(&self) -> Point {
        for _ in 0..5 {
            let m0 = Arc::new(&self.outline[0], &self.outline[1]).middle_random();
            let m1 = Arc::new(&self.outline[1], &self.outline[2]).middle_random();
            let m2 = Arc::new(&self.outline[2], &self.outline[3]).middle_random();
            let m3 = Arc::new(&self.outline[3], &self.outline[4]).middle_random();
            let d0 = Arc::new(&m0, &m2);
            let d1 = Arc::new(&m1, &m3);
            if let Some(intersection) = d0.intersection(&d1) {
                return intersection;
            }
        }
        panic!("no midpoint found :(");
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

pub struct Tiling {}

impl Tiling {
    pub fn base_tiling() -> Vec<ConvecQuadrilateral> {
        let np = Point::from_geodetic(90.0, 0.0);
        let sp = Point::from_geodetic(-90.0, 0.0);
        let mid_ring: Vec<f64> = vec![180.0, -90.0, 0.0, 90.0, 180.0];
        let mid_ring: Vec<Point> = mid_ring
            .iter()
            .map(|&lon| Point::from_geodetic(0.0, lon))
            .collect();
        let upper_ring: Vec<Point> = mid_ring
            .iter()
            .map(|mid| Arc::new(&mid, &np).middle())
            .collect();
        let lower_ring: Vec<Point> = mid_ring
            .iter()
            .map(|mid| Arc::new(&mid, &sp).middle())
            .collect();
        let mid_ring: Vec<f64> = vec![-135.0, -45.0, 45.0, 135.0, -135.0];
        let mid_ring: Vec<Point> = mid_ring
            .iter()
            .map(|&lon| Point::from_geodetic(0.0, lon))
            .collect();

        let mut base_pixels = Vec::new();

        for i in 0..4 {
            let polygon = ConvecQuadrilateral::new(&vec![
                upper_ring[i].clone(),
                mid_ring[i].clone(),
                upper_ring[i + 1].clone(),
                np.clone(),
                upper_ring[i].clone(),
            ]);
            base_pixels.push(polygon);

            let polygon = ConvecQuadrilateral::new(&vec![
                lower_ring[i].clone(),
                sp.clone(),
                lower_ring[i + 1].clone(),
                mid_ring[i].clone(),
                lower_ring[i].clone(),
            ]);
            base_pixels.push(polygon);

            let polygon = ConvecQuadrilateral::new(&vec![
                mid_ring[i].clone(),
                lower_ring[i + 1].clone(),
                mid_ring[i + 1].clone(),
                upper_ring[i + 1].clone(),
                mid_ring[i].clone(),
            ]);
            base_pixels.push(polygon);
        }
        base_pixels
    }
}
