#![allow(clippy::inline_always)]
/// Type conversions
mod convert;
/// Traits implementations
mod impls;
/// Iterator tools module
mod iter;
/// Hex ring utils
mod rings;
mod tests;

pub(crate) use iter::ExactSizeIsoIterator;
pub use iter::IsoIterExt;

use crate::direction::{way::DirectionWay, EdgeDirection, VertexDirection};
use glam::{IVec2, IVec3, Vec2, Vec3};
use std::time::Instant;
use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt::Debug,
};

//TODO change values to f32
/// Isometric coordinates
#[derive(Copy, Clone, Default, Eq, PartialEq)]
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "packed", repr(C))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Iso {
    /// Position in the x coordinate (bottom-left to top-right)
    pub x: i32,
    /// Position in the y coordinate (top-left to bottom-right)
    pub y: i32,
    /// Position in the z coordinate (depth)
    pub z: i32,
}

#[inline(always)]
#[must_use]
/// Instantiates a new isometric tile from coordinates
///
/// # Example
///
/// ```rust
/// # use iso::*;
/// let coord = iso(3, 5, 0);
/// assert_eq!(coord.x, 3);
/// assert_eq!(coord.y, 5);
/// assert_eq!(coord.z, 0);
/// ```
pub const fn iso(x: i32, y: i32, z: i32) -> Iso {
    Iso::new(x, y, z)
}

impl Iso {
    /// (0, 0)
    pub const ORIGIN: Self = Self::ZERO;
    /// (0, 0)
    pub const ZERO: Self = Self::new(0, 0, 0);
    /// (1, 1)
    pub const ONE: Self = Self::new(1, 1, 0);
    /// (-1, -1)
    pub const NEG_ONE: Self = Self::new(-1, -1, 0);

    /// +X (1, 0)
    pub const X: Self = Self::new(1, 0, 0);
    /// -X (-1, 0)
    pub const NEG_X: Self = Self::new(-1, 0, 0);
    /// +Y (0, 1)
    pub const Y: Self = Self::new(0, 1, 0);
    /// -Y (0, -1)
    pub const NEG_Y: Self = Self::new(0, -1, 0);

    // TODO review
    /// Unit vectors that increase the X axis in clockwise order
    pub const INCR_X: [Self; 2] = [Self::new(1, 0, 0), Self::new(1, -1, 0)];
    /// Unit vectors that increase the Y axis in clockwise order
    pub const INCR_Y: [Self; 2] = [Self::new(0, 1, 0), Self::new(-1, 1, 0)];
    /// Unit vectors that increase the Z axis in clockwise order
    pub const INCR_Z: [Self; 2] = [Self::new(0, 0, 0), Self::new(0, 0, 0)];

    /// Unit vectors that decrease the X axis in clockwise order
    pub const DECR_X: [Self; 2] = [Self::new(-1, 0, 0), Self::new(-1, 1, 0)];
    /// Unit vectors that decrease the Y axis in clockwise order
    pub const DECR_Y: [Self; 2] = [Self::new(0, -1, 0), Self::new(1, -1, 0)];
    /// Unit vectors that decrease the Z axis in clockwise order
    pub const DECR_Z: [Self; 2] = [Self::new(0, 0, 0), Self::new(0, 0, 0)];

    /// Isometric edge neighbor coordinates array, following [`EdgeDirection`]
    /// order
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
    /// Cubic coordinates:
    ///
    /// ```txt
    ///            /\  /\
    ///   (-1, 0) /2 \/3 \ (0, 1)
    ///           \  /\  /
    ///            \/  \/
    ///            /\  /\
    ///   (0, -1) /1 \/0 \ (1, 0)
    ///          \  /\  /
    ///           \/  \/
    /// ```
    pub const NEIGHBORS_COORDS: [Self; 4] = [
        Self::new(1, 0, 0),
        Self::new(0, -1, 0),
        Self::new(-1, 0, 0),
        Self::new(0, 1, 0),
    ];

    /// Isometric diagonal neighbor coordinates array, following
    /// [`VertexDirection`] order
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
    pub const DIAGONAL_COORDS: [Self; 4] = [
        Self::new(1, -1, 0),
        Self::new(-1, -1, 0),
        Self::new(-1, 1, 0),
        Self::new(1, 1, 0),
    ];

    #[inline(always)]
    #[must_use]
    /// Instantiates a new isometric tile from coordinates
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::new(3, 5, 0);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 5);
    /// assert_eq!(coord.z, 0);
    /// ```
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    // TODO I don't like this splat
    #[inline]
    #[must_use]
    /// Instantiates a new isometric tile with all coordinates set to `v`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::splat(3);
    /// assert_eq!(coord.x, 3);
    /// assert_eq!(coord.y, 3);
    /// assert_eq!(coord.z, 3);
    /// ```
    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v, z: v }
    }

    #[inline]
    #[must_use]
    /// `x` coordinate
    pub const fn x(self) -> i32 {
        self.x
    }

    #[inline]
    #[must_use]
    /// `y` coordinate
    pub const fn y(self) -> i32 {
        self.y
    }

    #[inline]
    #[must_use]
    /// `z` coordinate
    pub const fn z(self) -> i32 {
        self.z
    }

    // TODO too many from/to clean up
    #[inline]
    #[must_use]
    /// Creates a [`Iso`] from an array
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let p = Iso::from_array([3, 5, 0]);
    /// assert_eq!(p.x, 3);
    /// assert_eq!(p.y, 5);
    /// assert_eq!(p.z, 0);
    /// ```
    pub const fn from_array([x, y, z]: [i32; 3]) -> Self {
        Self::new(x, y, z)
    }

    #[inline]
    #[must_use]
    /// Converts `self` to an array as `[x, y]`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::new(3, 5, 0);
    /// let [x, y, z] = coord.to_array();
    /// assert_eq!(x, 3);
    /// assert_eq!(y, 5);
    /// assert_eq!(z, 0);
    /// ```
    pub const fn to_array(self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    /// Converts `self` to an [`f32`] array as `[x, y, z]`
    pub const fn to_array_f32(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z as f32]
    }

    #[inline]
    #[must_use]
    /// Converts `self` to cubic coordinates array as `[x, y, z]`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(3, 5);
    /// let [x, y, z] = coord.to_cubic_array();
    /// assert_eq!(x, 3);
    /// assert_eq!(y, 5);
    /// assert_eq!(z, -3 - 5);
    /// ```
    pub const fn to_cubic_array(self) -> [i32; 3] {
        [self.x, self.y, self.z()]
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    /// Converts `self` to cubic [`f32`] coordinates array as `[x, y, z]`
    pub const fn to_cubic_array_f32(self) -> [f32; 3] {
        [self.x as f32, self.y as f32, self.z() as f32]
    }
    /// Creates a [`Iso`] from the first 3 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[i32]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [i32]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
    }

    #[must_use]
    #[inline]
    /// Converts `self` to an [`IVec2`].
    /// This operation is a direct mapping of coordinates, no hex to square
    /// coordinates are performed. To convert hex coordinates to world space
    /// use [`HexLayout`]
    ///
    /// [`HexLayout`]: crate::HexLayout
    pub const fn as_ivec2(self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[must_use]
    #[inline]
    /// Converts `self` to an [`IVec3`] using cubic coordinates.
    /// This operation is a direct mapping of coordinates.
    /// To convert hex coordinates to world space use [`IsoLayout`]
    ///
    /// [`IsoLayout`]: crate::IsoLayout
    pub const fn as_ivec3(self) -> IVec3 {
        IVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    // TODO delete when not needed
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    #[inline]
    /// Converts `self` to a [`Vec3`].
    /// This operation is a direct mapping of coordinates.
    /// To convert iso coordinates to world space use [`IsoLayout`]
    ///
    /// [`IsoLayout`]: crate::IsoLayout
    pub const fn as_vec2(self) -> Vec2 {
        Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    #[inline]
    /// Converts `self` to a [`Vec3`].
    /// This operation is a direct mapping of coordinates.
    /// To convert iso coordinates to world space use [`IsoLayout`]
    ///
    /// [`IsoLayout`]: crate::IsoLayout
    pub const fn as_vec3(self) -> Vec3 {
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// Negates the coordinate, giving its reflection (symmetry) around the
    /// origin.
    ///
    /// [`Iso`] implements [`Neg`] (`-` operator) but this method is `const`.
    ///
    /// [`Neg`]: std::ops::Neg
    pub const fn const_neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: self.z,
        }
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// adds `self` and `other`.
    ///
    /// [`Iso`] implements [`Add`] (`+` operator) but this method is `const`.
    ///
    /// [`Add`]: std::ops::Add
    pub const fn const_add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// substracts `self` and `rhs`.
    ///
    /// [`Iso`] implements [`Sub`] (`-` operator) but this method is `const`.
    ///
    /// [`Sub`]: std::ops::Sub
    pub const fn const_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    /// Rounds floating point coordinates to [`Hex`].
    /// This method is used for operations like multiplications and divisions
    /// with floating point numbers.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let point = [0.6, 10.2, 0.5];
    /// let coord = Iso::round(point);
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 10);
    /// assert_eq!(coord.z, 1);
    /// ```
    pub fn round([mut x, mut y, z]: [f32; 3]) -> Self {
        let [mut x_r, mut y_r] = [x.round(), y.round()];
        x -= x_r;
        y -= y_r;
        if x.abs() >= y.abs() {
            x_r += 0.5_f32.mul_add(y, x).round();
        } else {
            y_r += 0.5_f32.mul_add(x, y).round();
        }
        Self::new(x_r as i32, y_r as i32, z as i32)
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// Computes the absolute value of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::new(-1, -32, -5).abs();
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 32);
    /// assert_eq!(coord.z, 5);
    /// ```
    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    // TODO Do I need this?
    /// Returns a vector containing the minimum values for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y),
    /// ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    // TODO Do I need this?
    /// Returns a vector containing the maximum values for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y),
    /// ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    // TODO Do I need this?
    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub const fn dot(self, rhs: Self) -> i32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// Returns a [`Iso`] with elements representing the sign of `self`.
    ///
    ///  - `0` if the number is zero
    ///  - `1` if the number is positive
    ///  - `-1` if the number is negative
    pub const fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    #[doc(alias = "magnitude")]
    /// Computes coordinates length as a signed integer.
    /// The length of a [`Iso`] coordinate is equal to its distance from the
    /// origin.
    ///
    /// See [`Self::ulength`] for the unsigned version
    ///
    /// # Example
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::new(10, 0, 0);
    /// assert_eq!(coord.length(), 10);
    /// ```
    pub const fn length(self) -> i32 {
        let [x, y, z] = [self.x.abs(), self.y.abs(), self.z().abs()];
        if x >= y && x >= z {
            x
        } else if y >= x && y >= z {
            y
        } else {
            z
        }
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    #[doc(alias = "unsigned_length")]
    /// Computes coordinates length as an unsigned integer
    /// The length of a [`Hex`] coordinate is equal to its distance from the
    /// origin.
    ///
    /// See [`Self::length`] for the signed version
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let coord = Hex::new(10, 0);
    /// assert_eq!(coord.ulength(), 10);
    /// ```
    pub const fn ulength(self) -> u32 {
        let [x, y, z] = [
            self.x.unsigned_abs(),
            self.y.unsigned_abs(),
            self.z().unsigned_abs(),
        ];
        if x >= y && x >= z {
            x
        } else if y >= x && y >= z {
            y
        } else {
            z
        }
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// Computes the distance from `self` to `rhs` in hexagonal space as a
    /// signed integer
    ///
    /// See [`Self::unsigned_distance_to`] for the unsigned version
    pub const fn distance_to(self, rhs: Self) -> i32 {
        self.const_sub(rhs).length()
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// Computes the distance from `self` to `rhs` in hexagonal space as an
    /// unsigned integer
    ///
    /// See [`Self::distance_to`] for the signed version
    pub const fn unsigned_distance_to(self, rhs: Self) -> u32 {
        self.const_sub(rhs).ulength()
    }

    // TODO Do I need this?
    #[inline]
    #[must_use]
    /// Retrieves the hexagonal neighbor coordinates matching the given
    /// `direction`
    pub const fn neighbor_coord(direction: EdgeDirection) -> Self {
        direction.into_iso()
    }

    #[inline]
    #[must_use]
    /// Retrieves the diagonal neighbor coordinates matching the given
    /// `direction`
    pub const fn diagonal_neighbor_coord(direction: VertexDirection) -> Self {
        direction.into_iso()
    }

    pub(crate) const fn add_dir(self, direction: EdgeDirection) -> Self {
        self.const_add(Self::neighbor_coord(direction))
    }

    pub(crate) const fn add_diag_dir(self, direction: VertexDirection) -> Self {
        self.const_add(Self::diagonal_neighbor_coord(direction))
    }

    #[inline]
    #[must_use]
    /// Retrieves the neighbor coordinates matching the given `direction`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::new(10, 5, 0);
    /// let bottom = coord.neighbor(EdgeDirection::BOTTOM_RIGHT);
    /// assert_eq!(bottom, Hex::new(11, 5, 0));
    /// ```
    pub const fn neighbor(self, direction: EdgeDirection) -> Self {
        self.const_add(Self::neighbor_coord(direction))
    }

    #[inline]
    #[must_use]
    /// Retrieves the diagonal neighbor coordinates matching the given
    /// `direction`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::new(10, 5,0);
    /// let bottom = coord.diagonal_neighbor(VertexDirection::RIGHT);
    /// assert_eq!(bottom, Hex::new(11, 6));
    /// ```
    pub const fn diagonal_neighbor(self, direction: VertexDirection) -> Self {
        self.const_add(Self::diagonal_neighbor_coord(direction))
    }

    #[inline]
    #[must_use]
    /// Retrieves the direction of the given neighbor. Will return `None` if
    /// `other` is not a neighbor of `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use iso::*;
    /// let coord = Iso::new(10, 5, 0);
    /// let bottom = coord.neighbor(EdgeDirection::BOTTOM_RIGHT);
    /// let dir = coord.neighbor_direction(bottom).unwrap();
    /// assert_eq!(dir, EdgeDirection::FLAT_BOTTOM);
    /// ```
    pub fn neighbor_direction(self, other: Self) -> Option<EdgeDirection> {
        EdgeDirection::iter().find(|&dir| self.neighbor(dir) == other)
    }

    #[must_use]
    /// Find in which [`VertexDirection`] wedge `rhs` is relative to `self`.
    ///
    /// > This method can be innaccurate in case of a *tie* between directions,
    /// > prefer
    /// using [`Self::diagonal_way_to`] instead
    pub fn main_diagonal_to(self, rhs: Self) -> VertexDirection {
        self.diagonal_way_to(rhs).unwrap()
    }

    // TODO continue
    #[must_use]
    /// Find in which [`VertexDirection`] wedge `rhs` is relative to `self`
    pub fn diagonal_way_to(self, rhs: Self) -> DirectionWay<VertexDirection> {
        let [x, y, z] = (rhs - self).to_cubic_array();
        let [xa, ya, za] = [x.abs(), y.abs(), z.abs()];
        match xa.max(ya).max(za) {
            v if v == xa => {
                DirectionWay::way_from(x < 0, xa == ya, xa == za, VertexDirection::DOWN)
            }
            _ => DirectionWay::way_from(y < 0, ya == za, ya == xa, VertexDirection::TOP),
        }
    }

    /// Find in which [`EdgeDirection`] wedge `rhs` is relative to `self`
    ///
    /// > This method can be innaccurate in case of a *tie* between directions,
    /// > prefer
    /// using [`Self::way_to`] for accuracy
    #[must_use]
    pub fn main_direction_to(self, rhs: Self) -> EdgeDirection {
        self.way_to(rhs).unwrap()
    }

    #[must_use]
    /// Find in which [`EdgeDirection`] wedge `rhs` is relative to `self`
    pub fn way_to(self, rhs: Self) -> DirectionWay<EdgeDirection> {
        let [x, y, z] = (rhs - self).to_cubic_array();
        let [x, y, z] = [y - x, z - y, x - z];
        let [xa, ya, za] = [x.abs(), y.abs(), z.abs()];
        match xa.max(ya).max(za) {
            v if v == xa => {
                DirectionWay::way_from(x < 0, xa == ya, xa == za, EdgeDirection::BOTTOM_RIGHT)
            }
            _ => DirectionWay::way_from(z < 0, za == xa, za == ya, EdgeDirection::TOP_RIGHT),
        }
    }

    #[inline]
    #[must_use]
    /// Retrieves all 4 neighbor coordinates around `self`
    pub fn all_neighbors(self) -> [Self; 4] {
        Self::NEIGHBORS_COORDS.map(|n| self.const_add(n))
    }

    #[inline]
    #[must_use]
    /// Retrieves all 4 neighbor diagonal coordinates around `self`
    pub fn all_diagonals(self) -> [Self; 4] {
        Self::DIAGONAL_COORDS.map(|n| self.const_add(n))
    }

    #[inline]
    #[must_use]
    #[doc(alias = "ccw")]
    /// Rotates `self` around [`Hex::ZERO`] counter clockwise (by -60 degrees)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let p = Hex::new(1, 2);
    /// assert_eq!(p.counter_clockwise(), Hex::new(3, -1));
    /// ```
    pub const fn counter_clockwise(self) -> Self {
        Self::new(-self.z(), -self.x, 0)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` counter clockwise (by -60 degrees)
    pub const fn ccw_around(self, center: Self) -> Self {
        self.const_sub(center).counter_clockwise().const_add(center)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around [`Hex::ZERO`] counter clockwise by `m` (by `-60 *
    /// m` degrees)
    pub const fn rotate_ccw(self, m: u32) -> Self {
        match m % 6 {
            1 => self.counter_clockwise(),
            2 => self.counter_clockwise().counter_clockwise(),
            3 => self.const_neg(),
            4 => self.clockwise().clockwise(),
            5 => self.clockwise(),
            _ => self,
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` counter clockwise by `m` (by `-60 * m`
    /// degrees)
    pub const fn rotate_ccw_around(self, center: Self, m: u32) -> Self {
        self.const_sub(center).rotate_ccw(m).const_add(center)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "cw")]
    /// Rotates `self` around [`Hex::ZERO`] clockwise (by 60 degrees)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// let p = Hex::new(1, 2);
    /// assert_eq!(p.clockwise(), Hex::new(-2, 3));
    /// ```
    pub const fn clockwise(self) -> Self {
        Self::new(-self.y, -self.z(), 0)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` clockwise (by 60 degrees)
    pub const fn cw_around(self, center: Self) -> Self {
        self.const_sub(center).clockwise().const_add(center)
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around [`Hex::ZERO`] clockwise by `m` (by `60 * m`
    /// degrees)
    pub const fn rotate_cw(self, m: u32) -> Self {
        match m % 6 {
            1 => self.clockwise(),
            2 => self.clockwise().clockwise(),
            3 => self.const_neg(),
            4 => self.counter_clockwise().counter_clockwise(),
            5 => self.counter_clockwise(),
            _ => self,
        }
    }

    #[inline]
    #[must_use]
    /// Rotates `self` around `center` clockwise by `m` (by `60 * m` degrees)
    pub const fn rotate_cw_around(self, center: Self, m: u32) -> Self {
        self.const_sub(center).rotate_cw(m).const_add(center)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_q")]
    /// Computes the reflection of `self` accross the `x` axis
    pub const fn reflect_x(self) -> Self {
        Self::new(self.x, self.z(), 0)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_r")]
    /// Computes the reflection of `self` accross the `y` axis
    pub const fn reflect_y(self) -> Self {
        Self::new(self.z(), self.y, 0)
    }

    #[inline]
    #[must_use]
    #[doc(alias = "reflect_s")]
    /// Computes the reflection of `self` accross the `z` axis
    pub const fn reflect_z(self) -> Self {
        Self::new(self.y, self.x, 0)
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    #[must_use]
    /// Computes all coordinates in a line from `self` to `other`.
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let start = Hex::ZERO;
    /// let end = Hex::new(5, 0);
    ///
    /// let line = start.line_to(end);
    /// assert_eq!(line.len(), 6);
    /// let line: Vec<Hex> = line.collect();
    /// assert_eq!(line.len(), 6);
    /// ````
    pub fn line_to(self, other: Self) -> impl ExactSizeIterator<Item = Self> {
        let d = other - self;
        let n = d.abs();
        let sign = d.signum();

        let count = (n.x.max(n.y) + 1) as usize;
        let mut p = self;

        let mut ix = 0;
        let mut iy = 0;
        let iter = std::iter::once(p).chain(std::iter::from_fn(move || {
            if ix >= n.x && iy >= n.y {
                return None;
            }
            if (ix * n.y + n.y / 2) < (iy * n.x + n.x / 2) {
                // next step is horizontal
                p.x += sign.x;
                ix += 1;
            } else {
                // next step is vertical
                p.y += sign.y;
                iy += 1;
            }
            Some(p)
        }));
        ExactSizeIsoIterator { iter, count }

        // TODO
        //let mut positions = HashSet::new();
        //let mut p = self;
        //positions.insert(p);

        //let mut ix = 0;
        //let mut iy = 0;
        //while ix < n.x || iy < n.y {
        //    if (ix * n.y + n.y / 2) < (iy * n.x + n.x / 2) {
        //        // next step is horizontal
        //        p.x += sign.x;
        //        ix += 1;
        //    } else {
        //        // next step is vertical
        //        p.y += sign.y;
        //        iy += 1;
        //    }
        //    positions.insert(p);
        //}
        //ExactSizeIsoIterator {
        //    count: positions.len(),
        //    iter: positions.into_iter(),
        //}
    }

    #[allow(clippy::cast_sign_loss)]
    #[must_use]
    /// Computes all coordinate in a two segment rectiline path from `self` to
    /// `other`
    ///
    /// # Arguments
    ///
    /// * `other` - The destination coordinate
    /// * `clockwise` - If set to `true` the line paths will be clockwise
    ///
    /// # Example
    /// ```rust
    /// # use hexx::*;
    /// let start = Hex::ZERO;
    /// let end = Hex::new(5, 0);
    ///
    /// let line = start.rectiline_to(end, true);
    /// assert_eq!(line.len(), 6);
    /// let line: Vec<Hex> = line.collect();
    /// assert_eq!(line.len(), 6);
    /// ````
    pub fn rectiline_to(self, other: Self, clockwise: bool) -> impl ExactSizeIterator<Item = Self> {
        let delta = other.const_sub(self);
        let count = delta.length();
        let mut dirs = self.main_diagonal_to(other).edge_directions();
        if !clockwise {
            dirs.rotate_left(1);
        }
        // The two directions to apply
        let [da, db] = dirs;
        // The amount of `da` is the distance between `delta` and the full projection of
        // `db`
        let proj_b = db * count;
        let ca = proj_b.distance_to(delta);

        let iter = std::iter::once(self).chain((0..count).scan(self, move |p, i| {
            if i <= ca {
                *p += da;
            } else {
                *p += db;
            }
            Some(*p)
        }));
        ExactSizeIsoIterator {
            iter,
            count: (count + 1) as usize,
        }
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the
    /// value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is
    /// `1.0`, the result will be equal to `rhs`. When `s` is outside of
    /// range `[0, 1]`, the result is linearly extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        let [start, end]: [Vec2; 2] = [self.as_vec2(), rhs.as_vec2()];
        start.lerp(end, s).into()
    }

    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    /// Retrieves all [`Hex`] around `self` in a given `range`.
    /// The number of returned coordinates is equal to `Hex::range_count(range)`
    ///
    /// > See also [`Hex::xrange`] to retrieve all coordinates excluding `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = hex(12, 34);
    /// assert_eq!(coord.range(0).len(), 1);
    /// assert_eq!(coord.range(1).len(), 7);
    /// ```
    pub fn range(self, range: u32) -> impl ExactSizeIterator<Item = Self> {
        let radius = range as i32;
        ExactSizeIsoIterator {
            iter: (-radius..=radius).flat_map(move |x| {
                let y_min = max(-radius, -x - radius);
                let y_max = min(radius, radius - x);
                (y_min..=y_max).map(move |y| self.const_add(Self::new(x, y, 0)))
            }),
            count: Self::range_count(range) as usize,
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    #[doc(alias = "excluding_range")]
    #[must_use]
    /// Retrieves all [`Hex`] around `self` in a given `range` except `self`.
    /// The number of returned coordinates is equal to
    /// `Hex::range_count(range) - 1`
    ///
    /// > See also [`Hex::range`] to retrieve all coordinates including `self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let coord = hex(12, 34);
    /// assert_eq!(coord.xrange(0).len(), 0);
    /// assert_eq!(coord.xrange(1).len(), 6);
    /// ```
    pub fn xrange(self, range: u32) -> impl ExactSizeIterator<Item = Self> {
        let iter = self.range(range);
        ExactSizeIsoIterator {
            count: iter.len().saturating_sub(1),
            iter: iter.filter(move |h| *h != self),
        }
    }

    /// Computes the coordinate of a lower resolution hexagon containing `self`
    /// of a given `radius`.
    /// The lower resolution coordinate can be considered *parent* of
    /// the contained higher resolution coordinates.
    /// The `radius` can be thought of as a *chunk size*, as if the grid was
    /// split in hexagonal chunks of that radius. The returned value are the
    /// coordinates of that chunk, in its own coordinates system.
    ///
    /// See the [source] documentation for more information
    ///
    /// > See also [`Self::to_higher_res`] and [`Self::to_local`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We define a coordinate
    /// let coord = hex(23, 45);
    /// // We take its *parent* in a coordinate system of size 5
    /// let parent = coord.to_lower_res(5);
    /// // We can then retrieve the center of that parent in the same system as `coord`
    /// let center = parent.to_higher_res(5);
    /// // Therefore the distance between the parent center and `coord` should be lower than 5
    /// assert!(coord.distance_to(center) <= 5);
    /// ```
    ///
    /// [source]: https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid
    #[must_use]
    #[allow(
        clippy::cast_possible_wrap,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation
    )]
    #[doc(alias = "downscale")]
    pub fn to_lower_res(self, radius: u32) -> Self {
        let [x, y, z] = self.to_cubic_array();
        let area = Self::range_count(radius) as f32;
        let shift = Self::shift(radius) as i32;
        let [x, y, z] = [
            ((y + shift * x) as f32 / area).floor() as i32,
            ((z + shift * y) as f32 / area).floor() as i32,
            ((x + shift * z) as f32 / area).floor() as i32,
        ];
        let [x, y] = [
            ((1 + x - y) as f32 / 3.0).floor() as i32,
            ((1 + y - z) as f32 / 3.0).floor() as i32,
            // ((1 + z - x) as f32 / 3.0).floor() as i32, -- z
        ];
        // debug_assert_eq!(z, -x - y);
        Self::new(x, y, 0)
    }

    /// Computes the center coordinates of `self` in a higher resolution system
    /// of a given `radius`.
    /// The higher resolution coordinate can be considered as a *child* of
    /// `self` as it is contained by it in a lower resolution coordinates
    /// system. The `radius` can be thought of as a *chunk size*, as if the
    /// grid was split in hexagonal chunks of that radius. The returned
    /// value are the coordinates of the center that chunk, in a higher
    /// resolution coordinates system.
    ///
    /// See the [source] documentation for more information
    ///
    /// > See also [`Self::to_lower_res`] and [`Self::to_local`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We define a coordinate
    /// let coord = hex(23, 45);
    /// // We take its *parent* in a coordinate system of size 5
    /// let parent = coord.to_lower_res(5);
    /// // We can then retrieve the center of that parent in the same system as `coord`
    /// let center = parent.to_higher_res(5);
    /// // Therefore the distance between the parent center and `coord` should be lower than 5
    /// assert!(coord.distance_to(center) <= 5);
    /// ```
    ///
    /// [source]: https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    #[doc(alias = "upscale")]
    pub const fn to_higher_res(self, radius: u32) -> Self {
        let range = radius as i32;
        let [x, y, z] = self.to_cubic_array();
        Self::new(x * (range + 1) - range * z, y * (range + 1) - range * x, 0)
    }

    /// Computes the local coordinates of `self` in a lower resolution
    /// coordinates system relative to its containing *parent* hexagon
    ///
    ///
    /// See the [source] documentation for more information
    ///
    /// > See also [`Self::to_lower_res`] and [`Self::to_local`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    ///
    /// // We define a coordinate
    /// let coord = hex(23, 45);
    /// // We can then retrieve the center of that hexagon in a higher res of size 5
    /// let center = coord.to_higher_res(5);
    /// // Therefore, the local coordinates of `center` relative to `coord` should be zero
    /// assert_eq!(center.to_local(5), Hex::ZERO);
    /// ```
    ///
    /// [source]: https://observablehq.com/@sanderevers/hexagon-tiling-of-an-hexagonal-grid
    #[must_use]
    pub fn to_local(self, radius: u32) -> Self {
        let upscale = self.to_lower_res(radius);
        let center = upscale.to_higher_res(radius);
        self.const_sub(center)
    }

    #[inline]
    #[must_use]
    /// Counts how many coordinates there are in the given `range`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// assert_eq!(Hex::range_count(15), 721);
    /// assert_eq!(Hex::range_count(0), 1);
    /// ```
    pub const fn range_count(range: u32) -> u32 {
        3 * range * (range + 1) + 1
    }

    /// Shift constant used for [hexmod] operations
    ///
    /// [hexmod]: https://observablehq.com/@sanderevers/hexmod-representation
    #[inline]
    #[must_use]
    pub(crate) const fn shift(range: u32) -> u32 {
        3 * range + 2
    }

    #[must_use]
    /// Wraps `self` in an hex range around the origin ([`Hex::ZERO`]).
    /// this allows for seamless *wraparound* hexagonal maps.
    /// See this [article] for more information.
    ///
    /// Use [`HexBounds`] for custom wrapping
    ///
    /// [`HexBounds`]: crate::HexBounds
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    pub fn wrap_in_range(self, range: u32) -> Self {
        self.to_local(range)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Debug for Iso {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Iso")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z())
            .finish()
    }
}
