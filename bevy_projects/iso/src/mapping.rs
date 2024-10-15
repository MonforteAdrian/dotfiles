/// mod Chunk handling
mod chunks;
/// mod Layer handling
mod layers;
/// mod public functions
mod utils;

use chunks::Chunk;
use layers::Layer;

pub use utils::generate_mesh_of_chunks;
pub use utils::get_sorted_tiles;

/// (columns, rows, layers)
pub const CHUNK_DIMENSIONS: (i32, i32, i32) = (12, 12, 4);
