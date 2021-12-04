pub trait IteratorExt: Sized + Iterator {
    fn pair_windows(self) -> PairWindows<Self, Self::Item>;
}

pub struct PairWindows<I, T> {
    iter: I,
    last: Option<T>,
}

impl<I> IteratorExt for I where I: Iterator {
    fn pair_windows(self) -> PairWindows<I, I::Item> {
        let mut iter = self;
        let last = iter.next();
        PairWindows { iter, last }
    }
}

impl<I, T> Iterator for PairWindows<I, T> where I: Iterator<Item = T>, T: Clone {
    type Item = [T; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last) = self.last.take() {
            if let Some(next) = self.iter.next() {
                self.last = Some(next.clone());
                return Some([last, next]);
            }
        }
        None
    }
}
