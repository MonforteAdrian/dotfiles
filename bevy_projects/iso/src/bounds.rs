use crate::Iso;

/// Hexagonal bounds utils, represented as a center and radius.
/// This type can be defined manually or from a [`Hex`] iterator.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let iter = Hex::ZERO.line_to(Hex::new(123, -456));
/// // You can compute the bounds of `iter`
/// let bounds: HexBounds = iter.collect();
/// ```
/// Bounds have [wraparound] features, useful for seamless or repeating maps.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let bounds = HexBounds::new(hex(1, 2), 10);
/// // Define a coordinate, even ouside of bounds
/// let point = Hex::new(100, 100);
/// assert!(!bounds.is_in_bounds(point));
/// // Retrieve the wrapped position in the map
/// let wrapped_point = bounds.wrap(point);
/// assert!(bounds.is_in_bounds(wrapped_point));
/// ```
///
/// [wraparound]: https://www.redblobgames.com/grids/hexagons/#wraparound
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct IsoBounds {
    /// bounds center
    pub center: Iso,
    /// bounds radius
    pub radius: u32,
}

impl IsoBounds {
    /// Instantiates new bounds from a `center` and `radius`
    #[inline]
    #[must_use]
    pub const fn new(center: Iso, radius: u32) -> Self {
        Self { center, radius }
    }

    /// Instantiates new bounds from a `radius` at [`Hex::ZERO`]
    #[inline]
    #[must_use]
    pub const fn from_radius(radius: u32) -> Self {
        Self {
            center: Iso::ZERO,
            radius,
        }
    }

    /// Computes the bounds `min` and `max`
    #[inline]
    #[must_use]
    pub fn from_min_max(min: Iso, max: Iso) -> Self {
        let center = (min + max) / 2;
        let radius = center.unsigned_distance_to(max) / 2;
        Self { center, radius }
    }

    /// Computes the bounds for `radius` with all coordinates
    /// being positive.
    ///
    /// This can be used for efficient map storage in a 2D vector
    /// disallowing negative coordinates
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn positive_radius(radius: u32) -> Self {
        let center = Iso::splat(radius as i32);
        Self { center, radius }
    }

    #[inline]
    #[must_use]
    /// Checks if `rhs` is in bounds
    pub const fn is_in_bounds(&self, rhs: Iso) -> bool {
        self.center.unsigned_distance_to(rhs) <= self.radius
    }

    #[must_use]
    #[inline]
    #[doc(alias = "coords_count")]
    #[doc(alias = "len")]
    /// Returns the number of hexagons in bounds
    pub const fn hex_count(&self) -> usize {
        Iso::range_count(self.radius) as usize
    }

    #[doc(alias = "all_items")]
    #[must_use]
    /// Returns an iterator with all the coordinates in bounds
    pub fn all_coords(&self) -> impl ExactSizeIterator<Item = Iso> {
        self.center.range(self.radius)
    }

    /// Computes all coordinates in the intersection between `self` and `rhs`
    pub fn intersecting_with(self, rhs: Self) -> impl Iterator<Item = Iso> {
        let [start, end] = if self.radius > rhs.radius {
            [rhs, self]
        } else {
            [self, rhs]
        };
        start.all_coords().filter(move |h| end.is_in_bounds(*h))
    }

    /// Wraps `coord`, returning a new local coodinate inside of the bounds,
    /// relative to the `center`.
    ///
    /// > This allows for seamless *wraparound* hexagonal maps.
    /// > See this [article] for more information.
    ///
    ///  > See also [`Self::wrap`] for global wrapping
    ///
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    #[must_use]
    pub fn wrap_local(&self, coord: Iso) -> Iso {
        let coord = coord - self.center;
        coord.wrap_in_range(self.radius)
    }

    /// Wraps `coord`, returning a new coodinate inside of the bounds.
    ///
    /// > This allows for seamless *wraparound* hexagonal maps.
    /// > See this [article] for more information.
    ///
    ///  > See also [`Self::wrap_local`] for local wrapping
    ///
    /// [article]: https://www.redblobgames.com/grids/hexagons/#wraparound
    #[must_use]
    pub fn wrap(&self, coord: Iso) -> Iso {
        self.wrap_local(coord) + self.center
    }
}

impl FromIterator<Iso> for IsoBounds {
    fn from_iter<T: IntoIterator<Item = Iso>>(iter: T) -> Self {
        let mut min = Iso::new(i32::MAX, i32::MAX, i32::MAX);
        let mut max = Iso::new(i32::MIN, i32::MIN, i32::MIN);

        for hex in iter {
            min.x = min.x.min(hex.x);
            max.x = max.x.max(hex.x);
            min.y = min.y.min(hex.y);
            max.y = max.y.max(hex.y);
        }
        Self::from_min_max(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_bounds_work() {
        let bounds = HexBounds::new(Hex::new(-4, 23), 34);
        for h in bounds.all_coords() {
            assert!(bounds.is_in_bounds(h));
        }
    }

    #[test]
    fn intersecting_with() {
        let ba = HexBounds::new(Hex::ZERO, 3);
        let bb = HexBounds::new(Hex::new(4, 0), 3);
        let intersection = ba.intersecting_with(bb);
        assert_eq!(intersection.count(), 9);
    }

    #[test]
    fn wrapping_works() {
        let map = HexBounds::from_radius(3);

        assert_eq!(map.wrap(Hex::new(0, 4)), Hex::new(-3, 0));
        assert_eq!(map.wrap(Hex::new(4, 0)), Hex::new(-3, 3));
        assert_eq!(map.wrap(Hex::new(4, -4)), Hex::new(0, 3));
    }

    #[test]
    fn wrapping_outside_works() {
        let map = HexBounds::from_radius(2);

        assert_eq!(map.wrap(Hex::new(3, 0)), Hex::new(-2, 2));
        assert_eq!(map.wrap(Hex::new(5, 0)), Hex::new(0, 2));
        assert_eq!(map.wrap(Hex::new(6, 0)), Hex::new(-1, -1));

        assert_eq!(map.wrap(Hex::new(2, 3)), Hex::new(0, 0)); // mirror
        assert_eq!(map.wrap(Hex::new(4, 6)), Hex::new(0, 0));
    }

    #[test]
    fn positive_radius() {
        for radius in 0..100_u32 {
            let bounds = HexBounds::positive_radius(radius);
            let coords = bounds.all_coords();
            let fails: Vec<_> = coords.filter(|c| c.x < 0 || c.y < 0).collect();
            println!("{fails:#?}");
            assert!(fails.is_empty());
        }
    }
}
