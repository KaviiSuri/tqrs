mod display;
mod ext;

use display::TqrsDisplay;
pub use ext::TqrsIteratorExt;
use std::io::{self, Write};

pub struct Unbounded;

/// Bounded is the state of the Tqrs Iterator which represents an ExactSizeIterator
/// It records the len of the ExactSizeIterator to reliably print the bounds of the progress bar
pub struct Bounded {
    /// bound is the size of the ExactSizeIterator
    bound: usize,
    /// delims are the characters used for the bounds
    delims: (char, char),
}

/// Tqrs wraps core::iter::traits::Iterator and prints out the progress bar to stderr (for now, may be configurable in the future)
pub struct Tqrs<Iter, Bound> {
    /// iter is the Iterator that is being wrapped
    iter: Iter,
    /// i is the number of iterations that have been done 
    i: usize,
    /// Bound represents a TypeState, which can be Unbounded and Bounded. These store bound related state
    bound: Bound,
    /// bar is teh string that is print for one bar of progress
    bar: String,
}

impl<Iter> Tqrs<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbounded,
            bar: "#".to_string(),
        }
    }
}

impl<Iter> Tqrs<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    /// with_bound is a builder method that adds a bound to the Tqrs Iterator
    /// This enables printing of "[" and "]" in the "[####      ]" progress bar
    pub fn with_bound(self) -> Tqrs<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Tqrs {
            i: self.i,
            iter: self.iter,
            bar: self.bar,
            bound,
        }
    }
}

impl<Iter> Tqrs<Iter, Bounded> {
    /// with_delims is a builder method that configures a bounded iterator's endings e.g. '[' ']'
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<Iter, Bound> Tqrs<Iter, Bound>
where
    Bound: TqrsDisplay,
    Iter: Iterator,
{
    /// with_bar is a builder method that is used to configure the bar indicator in the progress bar
    /// e.g. '#' in "[####     ]"
    pub fn with_bar(mut self, bar: &str) -> Self {
        self.bar = bar.to_string();
        self
    }

    /// update_progress updates the progress by one and updates the progressbar on screen
    pub fn update_progress(&mut self) {
        eprint!("\r");
        self.i += 1;
        self.bound.display(self);
        io::stderr().flush().expect("Failed to flush stderr");
    }
}

impl<Iter, Bound> Iterator for Tqrs<Iter, Bound>
where
    Iter: Iterator,
    Bound: TqrsDisplay,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.iter.next();
        if result.is_some() {
            self.update_progress();
        } else {
            eprintln!();
        }
        result
    }
}
