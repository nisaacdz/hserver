use std::cmp::Ordering;
use std::ops::Bound;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LowerBound<T>(pub Bound<T>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpperBound<T>(pub Bound<T>);

impl<T: Ord> PartialOrd for LowerBound<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for LowerBound<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.0, &other.0) {
            (Bound::Unbounded, Bound::Unbounded) => Ordering::Equal,
            (Bound::Unbounded, _) => Ordering::Less,
            (_, Bound::Unbounded) => Ordering::Greater,
            (Bound::Included(a), Bound::Included(b)) => a.cmp(b),
            (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(b),
            (Bound::Included(a), Bound::Excluded(b)) => a.cmp(b).then(Ordering::Less),
            (Bound::Excluded(a), Bound::Included(b)) => a.cmp(b).then(Ordering::Greater),
        }
    }
}

impl<T: Ord> PartialOrd for UpperBound<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for UpperBound<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.0, &other.0) {
            (Bound::Unbounded, Bound::Unbounded) => Ordering::Equal,
            (Bound::Unbounded, _) => Ordering::Greater,
            (_, Bound::Unbounded) => Ordering::Less,
            (Bound::Included(a), Bound::Included(b)) => a.cmp(b),
            (Bound::Excluded(a), Bound::Excluded(b)) => a.cmp(b),
            (Bound::Included(a), Bound::Excluded(b)) => a.cmp(b).then(Ordering::Greater),
            (Bound::Excluded(a), Bound::Included(b)) => a.cmp(b).then(Ordering::Less),
        }
    }
}
