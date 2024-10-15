use crate::{mapping, Iso, ProjectionMatrix};
use glam::{Vec2, Vec3};

/// Isometric layout. This type is the bridge between your *world*/*pixel*
/// coordinate system and the isometric coordinate system.
///
/// # Example
///
/// ```rust
/// # use iso::*;
///
/// let layout = IsoLayout {
///     // We want only the top face of the cube
///     tile_type: ProjectionMatrix::default(),
///     // We define the world space origin equivalent of `Iso::ZERO` in tile space
///     origin: Vec2::new(1.0, 2.0, 0.0),
///     // We define the world space size of the tileagons
///     tile_size: Vec2::new(1.0, 1.0),
/// };
/// // You can now find the world positon (center) of any given tile
/// let world_pos = layout.tile_to_world_pos(Iso::ZERO);
/// // You can also find which tile is at a given world/screen position
/// let tile_pos = layout.world_pos_to_tile(Vec2::new(1.23, 45.678));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct IsoLayout {
    /// The iso type of the layout full cube or top face
    pub tile_type: ProjectionMatrix,
    /// The origin of the isometric representation in world/pixel space, usually
    /// [`Vec3::ZERO`]
    pub origin: Vec3,
    /// The size of individual tiles in world/pixel space
    pub tile_size: Vec2,
    /// The heighest layer
    pub top_layer: i32,
}

impl IsoLayout {
    #[must_use]
    #[inline]
    /// Computes isometric coordinates `iso` into world/pixel coordinates
    pub fn tile_to_world_pos(&self, iso: Iso) -> Vec3 {
        let [x, y, z] = self.tile_type.forward(iso.to_array_f32());
        Vec3::new(x * self.tile_size.x, y * self.tile_size.y, z) + self.origin
    }

    #[must_use]
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    /// Computes world/pixel coordinates `pos` into isometric coordinates
    pub fn world_pos_to_tile(&self, pos: Vec2) -> Vec<Iso> {
        (0..self.top_layer)
            .map(move |layer| {
                let point = Vec3::new(
                    (pos.x - self.origin.x) / self.tile_size.x,
                    (pos.y - self.origin.y - self.tile_size.y / 4.) / self.tile_size.y,
                    layer as f32,
                );
                let [x, y, z] = self.tile_type.inverse(point.to_array());
                let p = Vec3::new(x, y, z);
                Iso::round(p.to_array())
            })
            .collect::<Vec<Iso>>()
    }

    //    /// Returns the  world coordinate of the two edge vertices in clockwise
    //    /// order
    //    #[must_use]
    //    pub fn edge_coordinates(&self, edge: crate::GridEdge) -> [Vec2; 2] {
    //        let origin = self.tile_to_world_pos(edge.origin);
    //        edge.vertices()
    //            .map(|v| self.__vertex_coordinates(v) + origin)
    //    }
    //
    //    /// Returns the  world coordinate of all edge vertex pairs in clockwise
    //    /// order
    //    #[must_use]
    //    pub fn all_edge_coordinates(&self, tile: Iso) -> [[Vec2; 2]; 6] {
    //        let origin = self.tile_to_world_pos(tile);
    //        tile.all_edges().map(|edge| {
    //            edge.vertices()
    //                .map(|v| self.__vertex_coordinates(v) + origin)
    //        })
    //    }
    //
    //    /// Returns the world coordinate of the vertex
    //    #[must_use]
    //    pub fn vertex_coordinates(&self, vertex: crate::GridVertex) -> Vec2 {
    //        let origin = self.tile_to_world_pos(vertex.origin);
    //        self.__vertex_coordinates(vertex) + origin
    //    }
    //
    //    fn __vertex_coordinates(&self, vertex: crate::GridVertex) -> Vec2 {
    //        vertex.direction.unit_vector(self.tile_type) * self.tile_size
    //    }
}

impl Default for IsoLayout {
    fn default() -> Self {
        Self {
            tile_type: ProjectionMatrix::default(),
            origin: Vec3::ZERO,
            tile_size: Vec2::ONE,
            top_layer: mapping::CHUNK_DIMENSIONS.2,
        }
    }
}

// TODO test
