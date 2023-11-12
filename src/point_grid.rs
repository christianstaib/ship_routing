use crate::{Arc, Collides, Contains, ConvecQuadrilateral, Point, PointStatus, Polygon, Tiling};

#[derive(Clone)]
pub struct PointSpatialPartition {
    pub boundary: ConvecQuadrilateral,
    pub node_type: PointNodeType,
    pub max_size: usize,
    pub midpoint: Point,
    pub midpoint_flag: PointStatus,
}

#[derive(Clone)]
pub enum PointNodeType {
    Internal(Vec<PointSpatialPartition>), // four children
    Leaf(Vec<Point>),                     // a bucket of points
}

pub struct PointPlanetGrid {
    pub spatial_partition: PointSpatialPartition,
}

impl PointPlanetGrid {
    pub fn add_point(&mut self, point: &Point) {
        self.spatial_partition.add_point(point);
    }

    pub fn new(max_size: usize) -> PointPlanetGrid {
        let polygons = Tiling::base_tiling();
        PointPlanetGrid {
            spatial_partition: PointSpatialPartition::new_root(&polygons, max_size),
        }
    }

    pub fn get_points(&self, polygon: &Polygon) -> Vec<Point> {
        self.spatial_partition.get_points(polygon)
    }
}

impl PointSpatialPartition {
    pub fn new_root(polygons: &Vec<ConvecQuadrilateral>, max_size: usize) -> PointSpatialPartition {
        let boundary = ConvecQuadrilateral::new(&vec![
            Point::from_coordinate(0.0, 0.0),
            Point::from_coordinate(1.0, 1.0),
            Point::from_coordinate(1.0, -1.0),
            Point::from_coordinate(1.0, -1.0),
            Point::from_coordinate(0.0, 0.0),
        ]);
        let midpoint = boundary.get_midpoint();
        PointSpatialPartition {
            boundary,
            node_type: PointNodeType::Internal(
                polygons
                    .iter()
                    .cloned()
                    .map(|p| PointSpatialPartition::new_leaf(p, max_size))
                    .collect(),
            ),
            max_size,
            midpoint,
            midpoint_flag: PointStatus::Outside,
        }
    }

    pub fn new_leaf(boundary: ConvecQuadrilateral, max_size: usize) -> PointSpatialPartition {
        let midpoint = boundary.get_midpoint();
        PointSpatialPartition {
            boundary,
            node_type: PointNodeType::Leaf(Vec::with_capacity(max_size + 1)),
            max_size,
            midpoint,
            midpoint_flag: PointStatus::Outside,
        }
    }

    fn split(&mut self) {
        let mut points: Vec<Point> = Vec::new();
        if let PointNodeType::Leaf(old_points) = &self.node_type {
            points.extend(old_points);
        }
        self.node_type = PointNodeType::Internal(
            self.boundary
                .split()
                .into_iter()
                .map(|rectangle| PointSpatialPartition::new_leaf(rectangle, self.max_size))
                .collect(),
        );

        points.iter().for_each(|point| self.add_point(point));
    }

    fn add_point(&mut self, point: &Point) {
        let mut internals = vec![self];
        while let Some(parent) = internals.pop() {
            // needs to be done before the match block, as the match block pushes a mutable
            // reference to internals.
            if let PointNodeType::Leaf(points) = &mut parent.node_type {
                points.push(point.clone());
                if points.len() >= parent.max_size {
                    parent.split();
                }
                break;
            }
            if let PointNodeType::Internal(childs) = &mut parent.node_type {
                for child in childs.iter_mut() {
                    if child.boundary.contains(point) {
                        internals.push(child);
                        break;
                    }
                }
            }
        }
    }

    pub fn get_points(&self, polygon: &Polygon) -> Vec<Point> {
        match &self.node_type {
            PointNodeType::Internal(q) => q
                .iter()
                .filter(|q| q.boundary.collides(polygon))
                .map(|q| q.get_points(polygon))
                .flatten()
                .collect(),
            PointNodeType::Leaf(points) => points
                .iter()
                .filter(|&point| polygon.contains(point))
                .cloned()
                .collect(),
        }
    }

    pub fn count_points(&self) -> usize {
        match &self.node_type {
            PointNodeType::Internal(q) => q.iter().map(|q| q.count_points()).sum(),
            PointNodeType::Leaf(points) => points.len(),
        }
    }

    pub fn get_leaf_polygons(&self) -> Vec<Arc> {
        match &self.node_type {
            PointNodeType::Internal(q) => q.iter().flat_map(|q| q.get_leaf_polygons()).collect(),
            PointNodeType::Leaf(_) => self
                .boundary
                .outline
                .windows(2)
                .map(|arc| Arc::new(&arc[0], &arc[1])._make_good_line())
                .flatten()
                .collect(),
        }
    }
}
