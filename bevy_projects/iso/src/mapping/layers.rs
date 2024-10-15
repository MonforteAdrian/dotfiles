use crate::Iso;

/// Layer of a chunk parameters.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Layer {
    /// z coordinate of the layer
    pub z: i32,
    /// Iterator of the tiles of a 'Layer'
    pub tiles: Vec<Iso>,
}

impl Layer {
    /// Generates a chunk
    #[must_use]
    pub fn new(chunk_x: i32, chunk_y: i32, z: i32, columns: i32, rows: i32) -> Self {
        let mut y_offset = columns - 3;
        let mut x_offset = -2;
        let tiles = (0..rows)
            .rev()
            .flat_map(move |row| {
                if row % 2 == 0 {
                    x_offset -= 1;
                } else {
                    y_offset -= 1;
                }

                let mut x = x_offset + chunk_x * columns + chunk_y * columns / 2;
                let mut y = y_offset + chunk_x * -rows + chunk_y * columns / 2;

                (0..columns).map(move |_| {
                    let pos = Iso::new(x, y, z);
                    x += 1;
                    y -= 1;
                    pos
                })
            })
            .collect();
        Self { z, tiles }
    }
}
