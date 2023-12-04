mod point_spatial_partition;
mod polygon_spatial_partition;
mod tiling;

pub use crate::spatial_partition::point_spatial_partition::PointSpatialPartition;
pub use crate::spatial_partition::polygon_spatial_partition::PolygonSpatialPartition;
pub use crate::spatial_partition::tiling::ConvecQuadrilateral;
