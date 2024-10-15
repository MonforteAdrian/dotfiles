use crate::{direction::EdgeDirection, Iso};
use std::fmt::Debug;

/// All 4 possible diagonal/vertex directions in isometric space.
///
/// ```txt
///          /\
///         /2 \
///        /\  /\
///   -X  /  \/  \ +Y
///      /\  /\  /\
///     /1 \/  \/3 \
///     \  /\  /\  /
///      \/  \/  \/
///   -Y  \  /\  / +X
///        \/0 \/
///         \  /
///          \/
/// ```
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
///  - multiplied by an `i32`, returning a [`Iso`](crate::Iso) vector
///
/// Example:
/// ```rust
/// # use iso::*;
/// let direction = VertexDirection::FLAT_RIGHT;
/// assert_eq!(-direction, VertexDirection::FLAT_LEFT);
/// assert_eq!(direction >> 1, VertexDirection::FLAT_BOTTOM_RIGHT);
/// assert_eq!(direction << 1, VertexDirection::FLAT_TOP_RIGHT);
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
#[doc(alias = "DiagonalDirection")]
pub struct VertexDirection(pub(crate) u8);

impl VertexDirection {
    /// Direction towards `X, -Y`
    pub const X_NEG_Y: Self = Self(0);
    /// Direction to (1, -1)
    ///
    /// Represents "Down"
    pub const DOWN: Self = Self(0);
    /// Direction to (1, -1)
    ///
    /// Represents "South"
    pub const SOUTH: Self = Self(0);
    /// Direction to (1, -1)

    /// Direction towards `-X, -Y`
    pub const NEG_X_NEG_Y: Self = Self(1);
    /// Direction to (-1, -1)
    ///
    /// Represents "Left"
    pub const LEFT: Self = Self(1);
    /// Direction to (-1, -1)
    ///
    /// Represents "West"
    pub const WEST: Self = Self(1);
    /// Direction to (-1, -1)

    /// Direction towards `-X, Y`
    pub const NEG_X_Y: Self = Self(2);
    /// Direction to (-1, 1)
    ///
    /// Represents "TOP"
    pub const TOP: Self = Self(2);
    /// Direction to (-1, 1)
    ///
    /// Represents "North"
    pub const NORTH: Self = Self(2);
    /// Direction to (-1, 1)

    /// Direction towards `X, Y`
    pub const X_Y: Self = Self(3);
    /// Direction to (1, 1)
    ///
    /// Represents "Right"
    pub const RIGHT: Self = Self(3);
    /// Direction to (1, 1)
    ///
    /// Represents "East"
    pub const EAST: Self = Self(3);
    /// Direction to (1, 1)

    /// All 4 diagonal directions matching
    /// [`Iso::DIAGONAL_COORDS`](crate::Iso::DIAGONAL_COORDS)
    ///
    /// ```txt
    ///         2
    ///         /\
    ///     -X /  \ Y
    ///    1  /    \
    ///       \    / 3
    ///     -Y \  / X
    ///         \/
    ///         0
    /// ```
    pub const ALL_DIRECTIONS: [Self; 6] = [Self(0), Self(1), Self(2), Self(3), Self(4), Self(5)];

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

    /// Converts the direction to a hex coordinate
    #[must_use]
    #[inline]
    pub const fn into_iso(self) -> Iso {
        Iso::DIAGONAL_COORDS[self.0 as usize]
    }

    /// Computes the opposite direction of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// assert_eq!(VertexDirection::EAST.const_neg(), VertexDirection::WEST);
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
    /// assert_eq!(VertexDirection::EAST.clockwise(), VertexDirection::SOUTH);
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
    ///     VertexDirection::EAST.counter_clockwise(),
    ///     VertexDirection::NORTH
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
    /// assert_eq!(VertexDirection::X, VertexDirection::X.rotate_ccw(6));
    /// ```
    pub const fn rotate_ccw(self, offset: u8) -> Self {
        Self((self.0 + 6 - (offset % 6)) % 6)
    }

    #[must_use]
    #[inline]
    /// Rotates `self` clockwise by `offset` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(VertexDirection::X, VertexDirection::X.rotate_cw(6));
    /// ```
    pub const fn rotate_cw(self, offset: u8) -> Self {
        Self((self.0 + (offset % 6)) % 6)
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let diagonal = VertexDirection::RIGHT.direction_ccw();
    /// assert_eq!(diagonal, EdgeDirection::TOP);
    /// ```
    pub const fn direction_ccw(self) -> EdgeDirection {
        self.edge_ccw()
    }

    #[inline]
    #[must_use]
    /// Computes the counter clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let diagonal = VertexDirection::RIGHT.edge_ccw();
    /// assert_eq!(diagonal, EdgeDirection::TOP);
    /// ```
    pub const fn edge_ccw(self) -> EdgeDirection {
        EdgeDirection(self.counter_clockwise().0)
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let diagonal = VertexDirection::RIGHT.direction_cw();
    /// assert_eq!(diagonal, EdgeDirection::BOTTOM);
    /// ```
    pub const fn direction_cw(self) -> EdgeDirection {
        self.edge_cw()
    }

    #[inline]
    #[must_use]
    /// Computes the clockwise [`EdgeDirection`] neighbor of self.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let diagonal = VertexDirection::RIGHT.edge_cw();
    /// assert_eq!(diagonal, EdgeDirection::BOTTOM);
    // ```
    pub const fn edge_cw(self) -> EdgeDirection {
        EdgeDirection(self.0)
    }

    #[inline]
    #[must_use]
    /// Computes the two adjacent [`EdgeDirection`] in clockwise order
    pub const fn edge_directions(self) -> [EdgeDirection; 2] {
        [self.edge_ccw(), self.edge_cw()]
    }
}

impl From<VertexDirection> for Iso {
    fn from(value: VertexDirection) -> Self {
        value.into_iso()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for VertexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.into_iso();
        f.debug_struct("VertexDirection")
            .field("index", &self.0)
            .field("x", &c.x)
            .field("y", &c.y)
            .finish()
    }
}
