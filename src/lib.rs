#![cfg_attr(
    feature = "nightly",
    feature(
        trusted_len,
        trusted_fused,
        min_specialization,
        trusted_random_access,
        inplace_iteration,
        doc_auto_cfg
    )
)]
#![no_std]
use core::iter::FusedIterator;
#[cfg(feature = "nightly")]
use core::iter::{
    InPlaceIterable, SourceIter, TrustedLen, TrustedRandomAccess, TrustedRandomAccessNoCoerce,
};
/// Provides a hint for an iterator.
#[derive(Clone, Debug)]
pub struct Hinted<I> {
    /// Wrapped iterator.
    iter: I,
    /// Size hint, decreased on every call to [`next()`](Iterator::next).
    hint: usize,
}

/// Set the size hint for an iterator.
pub trait HintExt: Iterator
where
    Self: Sized,
{
    /// Sets the lower bound of [`Iterator::size_hint`]. Useful for collects etc.
    fn hinted(self, hint: usize) -> Hinted<Self> {
        Hinted { iter: self, hint }
    }
    /// Implements [`ExactSizeIterator`] for any iterator, with a len provided by you.
    /// This implements [`TrustedLen`](std::iter::TrustedLen) as well, so this function is unsafe.
    ///
    /// The resulting iterator will not return after the length has been reached.
    ///
    /// # Safety
    ///
    /// number of items *must* be `len`.
    unsafe fn has(self, len: usize) -> Exactly<Self> {
        Exactly { iter: self, len }
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
impl<I: DoubleEndedIterator> DoubleEndedIterator for Hinted<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.hint = self.hint.saturating_sub(1);
        self.iter.next_back()
    }
}

impl<I: FusedIterator> FusedIterator for Hinted<I> {}
#[cfg(feature = "nightly")]
unsafe impl<I: TrustedRandomAccess> TrustedRandomAccess for Hinted<I> {}
#[cfg(feature = "nightly")]
unsafe impl<I: TrustedRandomAccessNoCoerce> TrustedRandomAccessNoCoerce for Hinted<I> {
    const MAY_HAVE_SIDE_EFFECT: bool = true;
}
#[cfg(feature = "nightly")]
unsafe impl<I: SourceIter> SourceIter for Hinted<I> {
    type Source = I::Source;

    #[inline]
    unsafe fn as_inner(&mut self) -> &mut I::Source {
        // SAFETY: unsafe function forwarding to unsafe function with the same requirements
        unsafe { SourceIter::as_inner(&mut self.iter) }
    }
}
#[cfg(feature = "nightly")]
unsafe impl<I: InPlaceIterable> InPlaceIterable for Hinted<I> {
    const EXPAND_BY: Option<core::num::NonZero<usize>> = I::EXPAND_BY;
    const MERGE_BY: Option<core::num::NonZero<usize>> = I::MERGE_BY;
}

/// Provides a size for an iterator.
#[derive(Clone, Debug)]
pub struct Exactly<I> {
    /// Wrapped iterator.
    iter: I,
    /// Exact size, decreased on every call to [`next()`](Iterator::next).
    len: usize,
}

/// Set the size hint for an iterator.
impl<I> Iterator for Exactly<I>
where
    I: Iterator + Sized,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.len = unsafe { self.len.unchecked_sub(1) };
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I: Iterator> ExactSizeIterator for Exactly<I> {
    fn len(&self) -> usize {
        self.len
    }
}
impl<I: DoubleEndedIterator> DoubleEndedIterator for Exactly<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.len = unsafe { self.len.unchecked_sub(1) };
        self.iter.next_back()
    }
}
impl<I: FusedIterator> FusedIterator for Exactly<I> {}

// SAFETY: see [`HintExt::has`].
#[cfg(feature = "nightly")]
unsafe impl<I: Iterator> TrustedLen for Exactly<I> {}
#[cfg(feature = "nightly")]
unsafe impl<I: TrustedRandomAccess> TrustedRandomAccess for Exactly<I> {}
#[cfg(feature = "nightly")]
unsafe impl<I: TrustedRandomAccessNoCoerce> TrustedRandomAccessNoCoerce for Exactly<I> {
    const MAY_HAVE_SIDE_EFFECT: bool = true;
}
#[cfg(feature = "nightly")]
unsafe impl<I: SourceIter> SourceIter for Exactly<I> {
    type Source = I::Source;

    #[inline]
    unsafe fn as_inner(&mut self) -> &mut I::Source {
        // SAFETY: unsafe function forwarding to unsafe function with the same requirements
        unsafe { SourceIter::as_inner(&mut self.iter) }
    }
}
#[cfg(feature = "nightly")]
unsafe impl<I: InPlaceIterable> InPlaceIterable for Exactly<I> {
    const EXPAND_BY: Option<core::num::NonZero<usize>> = I::EXPAND_BY;
    const MERGE_BY: Option<core::num::NonZero<usize>> = I::MERGE_BY;
}
