use crate::mapping::Layer;

/// (columns, rows, layers)
pub const CHUNK_DIMENSIONS: (i32, i32, i32) = (12, 12, 4);

/// Chunk parameters.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Chunk {
    /// x coordinate of the chunk
    pub x: i32,
    /// y coordinate of the chunk
    pub y: i32,
    /// Iterator of the layers of the 'Chunk'
    pub layers: Vec<Layer>,
}

impl Chunk {
    /// Generates a new chunk with given x and y coordinates
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        let layers = (0..CHUNK_DIMENSIONS.2)
            .map(|z| Layer::new(x, y, z, CHUNK_DIMENSIONS.0, CHUNK_DIMENSIONS.1))
            .collect();
        Self { x, y, layers }
    }
}
