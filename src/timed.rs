use std::{iter::FusedIterator, time::{Duration, Instant}};


/// Tracks and returns durations of each call to `next()`.
/// 
/// See [`crate::Itertools::timed()`] for more details.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct TimedIterator<I> {
    iter: I,
    none_time: Option<Duration>,
}

impl<I: Iterator> TimedIterator<I> {
    /// Creates a new TimedIterator that times the given input
    /// iterator's calls to `next()` when iterated over.
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            none_time: None,
        }
    }

    /// If the inner iterator has returned `None`, then retrieve the time it
    /// took for that call to `next()`; otherwise return `None`. Note that
    /// this is unambiguous because once the inner iterator returns `None`
    /// once, it is never called again.
    pub fn none_time(&self) -> Option<Duration> {
        self.none_time
    }
}

impl<I: Iterator> Iterator for TimedIterator<I> {
    type Item = (I::Item, Duration);

    fn next(&mut self) -> Option<Self::Item> {
        if self.none_time.is_none() {
            let start = Instant::now();
            let item = self.iter.next();
            let duration = start.elapsed();
            if let Some(item) = item {
                Some((item, duration))
            } else {
                self.none_time = Some(duration);
                None
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I: Iterator> FusedIterator for TimedIterator<I> {}
impl<I: ExactSizeIterator> ExactSizeIterator for TimedIterator<I> {}
