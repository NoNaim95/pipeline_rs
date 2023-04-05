use crate::pipes::ReceivePipe;

pub struct ReceivePipeImpl<T, F: Fn() -> T>(F);

impl<T, F: Fn() -> T> ReceivePipeImpl<T, F> {
    pub fn new(f: F) -> Self {
        ReceivePipeImpl(f)
    }

    pub fn iter(&mut self) -> Iter<T, F> {
        Iter(self)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: Fn() -> T> ReceivePipe<T> for ReceivePipeImpl<T, F> {
    fn recv(&mut self) -> T {
        self.0()
    }
}

impl<T, F: Fn() -> T> From<F> for ReceivePipeImpl<T, F> {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

impl<T, F: Fn() -> Option<T>> ReceivePipeImpl<Option<T>, F> {
    pub fn try_iter(&mut self) -> TryIter<T, F> {
        TryIter(self)
    }

    pub fn into_try_iter(self) -> IntoTryIter<T, F> {
        IntoTryIter(self)
    }
}

pub struct TryIter<'a, T, F: Fn() -> Option<T>>(pub &'a mut ReceivePipeImpl<Option<T>, F>);

impl<'a, T, F: Fn() -> Option<T>> Iterator for TryIter<'a, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv()
    }
}

pub struct IntoTryIter<T, F: Fn() -> Option<T>>(pub ReceivePipeImpl<Option<T>, F>);

impl<T, F: Fn() -> Option<T>> Iterator for IntoTryIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv()
    }
}

pub struct Iter<'a, T, F: Fn() -> T>(pub &'a mut ReceivePipeImpl<T, F>);

impl<'a, T, F: Fn() -> T> Iterator for Iter<'a, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.recv())
    }
}

pub struct IntoIter<T, F: Fn() -> T>(pub ReceivePipeImpl<T, F>);

impl<T, F: Fn() -> T> Iterator for IntoIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.recv())
    }
}

impl<T, F: Fn() -> T> IntoIterator for ReceivePipeImpl<T, F> {
    type Item = T;
    type IntoIter = IntoIter<T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T, F: Fn() -> T> From<ReceivePipeImpl<T, F>> for IntoIter<T, F> {
    fn from(value: ReceivePipeImpl<T, F>) -> Self {
        Self(value)
    }
}
