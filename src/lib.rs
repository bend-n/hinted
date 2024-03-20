/// Provides a hint for an iterator.
pub struct Hinted<I: Iterator> {
    /// Wrapped iterator.
    pub iter: I,
    /// Size hint, decreased on every call to [`next()`](Iterator::next).
    pub hint: usize,
}

/// Set the size hint for an iterator.
pub trait HintExt: Iterator
where
    Self: Sized,
{
    fn hinted(self, hint: usize) -> Hinted<Self> {
        Hinted { iter: self, hint }
    }
}

impl<I: Iterator> HintExt for I {}

impl<I> Iterator for Hinted<I>
where
    I: Iterator + Sized,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.hint = self.hint.saturating_sub(1);
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.hint, self.iter.size_hint().1)
    }
}
