use crate::{Bounded, Tqrs, Unbounded};

/// TqrsDisplay is implemented by states of Tqrs that can be printed to screen
pub trait TqrsDisplay
where
    Self: Sized,
{
    fn display<Iter>(&self, tqrs: &Tqrs<Iter, Self>);
}

/// Unbounded implemetns TqrsDisplay
impl TqrsDisplay for Unbounded {
    fn display<Iter>(&self, tqrs: &Tqrs<Iter, Self>) {
        eprint!("{}", tqrs.bar.repeat(tqrs.i));
    }
}

/// Bounded implements TqrsDisplay and considers the bounds too
impl TqrsDisplay for Bounded {
    fn display<Iter>(&self, tqrs: &Tqrs<Iter, Self>) {
        if self.bound < tqrs.i {
            return;
        }
        eprint!(
            "{}{}{}{}",
            self.delims.0,
            tqrs.bar.repeat(tqrs.i),
            " ".repeat(self.bound - tqrs.i),
            self.delims.1
        );
    }
}
