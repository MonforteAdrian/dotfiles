use crate::{direction::VertexDirection, Iso};
use std::fmt::Debug;

/// All 4 possible neighbor/edge directions in isometric space.
///
/// ```txt
///        /\  /\
///   -X  /2 \/3 \ +Y
///       \  /\  /
///        \/  \/
///        /\  /\
///       /1 \/0 \
///   -Y \  /\  / +X
///       \/  \/
/// ```
///
/// See [`Iso::NEIGHBORS_COORDS`](crate::Iso::NEIGHBORS_COORDS)
///
///
/// ## Operations
///
/// Directions can be:
///  - rotated *clockwise* with:
///     - [`Self::clockwise`] and [`Self::rotate_cw`]
///     - The shift right `>>` operator
///  - rotated *counter clockwise* with:
///     - [`Self::counter_clockwise`] and [`Self::rotate_ccw`]
///     - The shift left `<<` operator
///  - negated using the minus `-` operator
///  - multiplied by an `i32`, returning a [`Hex`](crate::Hex) vector
///
/// Example:
/// ```rust
/// # use iso::*;
/// let direction = EdgeDirection::TOP;
/// assert_eq!(-direction, EdgeDirection::BOTTOM);
/// assert_eq!(direction >> 1, EdgeDirection::TOP_RIGHT);
/// assert_eq!(direction << 1, EdgeDirection::TOP_LEFT);
/// ```
///
/// ## Storage
///
/// Both [`EdgeDirection`] and [`VertexDirection`] store a u8 byte between 0 and
/// 5 as following:
///
/// ```txt
///         v2
///         /\
///  -X e2 /  \ e3 Y
///    v1 /    \
///       \    / v3
///  -Y e1 \  / e0 X
///         \/
///         v0
/// ```
///
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
#[repr(transparent)]
#[doc(alias = "Direction")]
pub struct EdgeDirection(pub(crate) u8);

impl EdgeDirection {
    /// Direction towards `X`
    pub const X: Self = Self(0);
    /// Direction to (1, 0)
    ///
    /// Represents "Bottom right" edge
    pub const BOTTOM_RIGHT: Self = Self(0);
    /// Direction to (1, 0)
    ///
    /// Represents "South East" edge
    pub const SOUTH_EAST: Self = Self(0);
    /// Direction to (1, 0)

    /// Direction towards `-Y`
    pub const NEG_Y: Self = Self(1);
    /// Direction to (0, -1)
    ///
    /// Represents "Bottom left" edge
    pub const BOTTOM_LEFT: Self = Self(1);
    /// Direction to (0, -1)
    ///
    /// Represents "South West" edge
    pub const SOUTH_WEST: Self = Self(1);
    /// Direction to (0, -1)

    /// Direction towards `-X`
    pub const NEG_X: Self = Self(2);
    /// Direction to (-1, 0)
    ///
    /// Represents "Top Left"
    pub const TOP_LEFT: Self = Self(2);
    /// Direction to (-1, 0)
    ///
    /// Represents "North West"
    pub const NORTH_WEST: Self = Self(2);
    /// Direction to (-1, 0)

    /// Direction towards `Y`
    pub const Y: Self = Self(3);
    /// Direction to (0, 1)
    ///
    /// Represents "Top right"
    pub const TOP_RIGHT: Self = Self(3);
    /// Direction to (0, 1)
    ///
    /// Represents "North East"
    pub const NORTH_EAST: Self = Self(3);
    /// Direction to (0, 1)

    /// All 4 isometric directions matching
    /// [`Iso::NEIGHBORS_COORDS`](crate::Iso::NEIGHBORS_COORDS)
    ///
    /// ```txt
    ///        /\  /\
    ///   -X  /2 \/3 \ +Y
    ///       \  /\  /
    ///        \/  \/
    ///        /\  /\
    ///       /1 \/0 \
    ///   -Y \  /\  / +X
    ///       \/  \/
    /// ```
    pub const ALL_DIRECTIONS: [Self; 4] = [Self(0), Self(1), Self(2), Self(3)];

    /// Iterates through all directions in clockwise order
    #[must_use]
    pub fn iter() -> impl ExactSizeIterator<Item = Self> {
        Self::ALL_DIRECTIONS.into_iter()
    }

    /// Returns the inner index of the edge direction, from 0 to 3
    #[must_use]
    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Converts the direction to a normalized hex coordinate
    #[must_use]
    #[inline]
    pub const fn into_iso(self) -> Iso {
        Iso::NEIGHBORS_COORDS[self.0 as usize]
    }

    /// Computes the opposite direction of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT.const_neg(),
    ///     EdgeDirection::BOTTOM_LEFT
    /// );
    /// ```
    #[must_use]
    #[inline]
    pub const fn const_neg(self) -> Self {
        Self((self.0 + 2) % 4)
    }

    /// Returns the next direction in clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT.clockwise(),
    ///     EdgeDirection::BOTTOM_RIGHT
    /// );
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "cw")]
    pub const fn clockwise(self) -> Self {
        Self((self.0 + 1) % 4)
    }

    /// Returns the next direction in counter clockwise order
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT.counter_clockwise(),
    ///     EdgeDirection::TOP_LEFT
    /// );
    /// ```
    #[must_use]
    #[inline]
    #[doc(alias = "ccw")]
    pub const fn counter_clockwise(self) -> Self {
        Self((self.0 + 3) % 4)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` counter clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// assert_eq!(
    ///     EdgeDirection::TOP_RIGHT,
    ///     EdgeDirection::TOP_RIGHT.rotate_ccw(6)
    /// );
    /// ```
    pub const fn rotate_ccw(self, offset: u8) -> Self {
        Self((self.0 + 4 - (offset % 4)) % 4)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(
    ///     EdgeDirection::FLAT_TOP,
    ///     EdgeDirection::FLAT_TOP.rotate_cw(6)
    /// );
    /// ```
    pub const fn rotate_cw(self, offset: u8) -> Self {
        Self((self.0 + (offset % 6)) % 6)
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.diagonal_ccw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_LEFT);
    /// ```
    pub const fn diagonal_ccw(self) -> VertexDirection {
        self.vertex_ccw()
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.vertex_ccw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_LEFT);
    /// ```
    pub const fn vertex_ccw(self) -> VertexDirection {
        VertexDirection(self.0)
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.diagonal_cw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn diagonal_cw(self) -> VertexDirection {
        self.vertex_cw()
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`VertexDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let diagonal = EdgeDirection::FLAT_TOP.vertex_cw();
    /// assert_eq!(diagonal, VertexDirection::FLAT_TOP_RIGHT);
    /// ```
    pub const fn vertex_cw(self) -> VertexDirection {
        VertexDirection(self.clockwise().0)
    }

    #[inline]
    #[must_use]
    /// Computes the two adjacent [`VertexDirection`] in clockwise order
    pub const fn vertex_directions(self) -> [VertexDirection; 2] {
        [self.vertex_ccw(), self.vertex_cw()]
    }
}

impl From<EdgeDirection> for Iso {
    fn from(value: EdgeDirection) -> Self {
        value.into_iso()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for EdgeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.into_iso();
        f.debug_struct("EdgeDirection")
            .field("index", &self.0)
            .field("x", &c.x)
            .field("y", &c.y)
            .finish()
    }
}
