use crate::math::{Point, Real};

use std::mem;

/// Closest points information.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClosestPoints {
    /// The two objects are intersecting.
    Intersecting,
    /// The two objects are non-intersecting but closer than a given user-defined distance.
    WithinMargin(Point<Real>, Point<Real>),
    /// The two objects are non-intersecting and further than a given user-defined distance.
    Disjoint,
}

impl ClosestPoints {
    /// Swaps the two points.
    pub fn flip(&mut self) {
        if let ClosestPoints::WithinMargin(ref mut p1, ref mut p2) = *self {
            mem::swap(p1, p2)
        }
    }

    /// Returns the result of swapping the two points if `self` is `WithinMargin`.
    pub fn flipped(&self) -> Self {
        if let ClosestPoints::WithinMargin(p1, p2) = *self {
            ClosestPoints::WithinMargin(p2, p1)
        } else {
            *self
        }
    }
}