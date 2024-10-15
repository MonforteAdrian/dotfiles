use crate::{Iso, IsoBounds};

/// Extension trait for iterators of [`Iso`]
pub trait IsoIterExt: Iterator {
    /// Method which takes an iterator and finds the mean (average) value.
    ///
    /// This method will return [`Iso::ZERO`] on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let mean = Iso::ZERO.range(10).average();
    /// assert_eq!(mean, Iso::ZERO);
    /// ```
    #[doc(alias = "mean")]
    fn average(self) -> Iso;

    /// Method which takes an iterator and finds the center (centroid) value.
    ///
    /// This method will return [`Iso::ZERO`] on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let center = Iso::ZERO.range(10).center();
    /// assert_eq!(center, Iso::ZERO);
    /// ```
    #[doc(alias = "centroid")]
    fn center(self) -> Iso;

    /// Method which takes an iterator and finds the bounds containing all
    /// elements.
    ///
    /// This method will return ([`Iso::ZERO`], 0) on an empty iterator
    ///
    /// # Example
    ///
    /// ```rust
    /// # use hexx::*;
    /// let bounds = Iso::ZERO.range(10).bounds();
    /// assert_eq!(bounds.center, Iso::ZERO);
    /// assert_eq!(bounds.radius, 10);
    /// ```
    fn bounds(self) -> IsoBounds;
}

impl<I: Iterator<Item = Iso>> IsoIterExt for I {
    fn average(self) -> Iso {
        let mut sum = Iso::ZERO;
        let mut count = 0;

        for hex in self {
            count += 1;
            sum += hex;
        }
        // Avoid division by zero
        sum / count.max(1)
    }

    fn center(self) -> Iso {
        self.bounds().center
    }

    fn bounds(self) -> IsoBounds {
        self.collect()
    }
}

/// Private container for a [`Iso`] [`Iterator`] of known size
#[derive(Debug, Clone)]
pub struct ExactSizeIsoIterator<I> {
    /// The inner iterator
    pub iter: I,
    /// The remaining iterator elements count
    pub count: usize,
}

impl<I> Iterator for ExactSizeIsoIterator<I>
where
    I: Iterator<Item = Iso>,
{
    type Item = Iso;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count.saturating_sub(1);
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl<I> ExactSizeIterator for ExactSizeIsoIterator<I> where I: Iterator<Item = Iso> {}
