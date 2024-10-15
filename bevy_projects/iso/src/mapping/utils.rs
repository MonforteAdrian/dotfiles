use crate::mapping::Chunk;
use crate::Iso;
use core::cmp::Reverse;

/// Generates the number of chunks needed to fill the window
#[must_use]
pub fn generate_mesh_of_chunks(max_x: i32, min_x: i32, max_y: i32, min_y: i32) -> Vec<Chunk> {
    (min_y..=max_y)
        .rev()
        .flat_map(move |y| (min_x..=max_x).map(move |x| Chunk::new(x, y)))
        .collect()
}

/// Function to collect all tiles from chunks and sort them by z, then y, then x
#[must_use]
pub fn get_sorted_tiles(chunks: Vec<Chunk>) -> Vec<Iso> {
    let mut tiles: Vec<Iso> = chunks
        .into_iter()
        .flat_map(|chunk| chunk.layers.into_iter().flat_map(|layer| layer.tiles))
        .collect();

    tiles.sort_by_key(|iso| (iso.z, Reverse(iso.y), Reverse(iso.x)));
    tiles
}
