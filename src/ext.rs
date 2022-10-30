use crate::{Tqrs, Unbounded};

/// TqrsIteratorExt is the extension trait that adds the `tqrs` method to Iterator
pub trait TqrsIteratorExt<'a>
where
    Self: Sized,
{
    fn tqrs(self) -> Tqrs<Self, Unbounded>;
}

/// Iterator implements TqrsIteratorExt
impl<'a, Iter> TqrsIteratorExt<'a> for Iter
where
    Iter: Iterator,
{
    fn tqrs(self) -> Tqrs<Self, Unbounded> {
        Tqrs::new(self)
    }
}
