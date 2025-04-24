/// Isometric neighbor/edge directions
mod edge_direction;
/// Trait implementations
mod impls;
/// Isometric vertex/diagonal directions
mod vertex_direction;
/// Direction way module
pub(crate) mod way;

pub use edge_direction::EdgeDirection;
pub use vertex_direction::VertexDirection;
pub use way::DirectionWay;
