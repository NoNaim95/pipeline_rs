use crate::pipes::{ReceivePipe, ReceivePipeMut};

pub struct ReceivePipeImpl<F>(F);

impl<T, F: FnMut() -> T> ReceivePipeImpl<F> {
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

impl<T, F: FnMut() -> T> ReceivePipeMut<T> for ReceivePipeImpl<F> {
    fn recv_mut(&mut self) -> T {
        self.0()
    }
}

impl<T, F: Fn() -> T> ReceivePipe<T> for ReceivePipeImpl<F> {
    fn recv(&self) -> T {
        self.0()
    }
}

impl<T, F: FnMut() -> Option<T>> ReceivePipeImpl<F> {
    pub fn try_iter(&mut self) -> TryIter<T, F> {
        TryIter(self)
    }

    pub fn into_try_iter(self) -> IntoTryIter<T, F> {
        IntoTryIter(self)
    }
}

pub struct TryIter<'a, T, F: FnMut() -> Option<T>>(pub &'a mut ReceivePipeImpl<F>);

impl<'a, T, F: FnMut() -> Option<T>> Iterator for TryIter<'a, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv_mut()
    }
}

pub struct IntoTryIter<T, F: FnMut() -> Option<T>>(pub ReceivePipeImpl<F>);

impl<T, F: FnMut() -> Option<T>> Iterator for IntoTryIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv_mut()
    }
}

pub struct Iter<'a, T, F: FnMut() -> T>(pub &'a mut ReceivePipeImpl<F>);

impl<'a, T, F: FnMut() -> T> Iterator for Iter<'a, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.recv_mut())
    }
}

pub struct IntoIter<T, F: FnMut() -> T>(pub ReceivePipeImpl<F>);

impl<T, F: FnMut() -> T> Iterator for IntoIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.recv_mut())
    }
}

impl<T, F: FnMut() -> T> IntoIterator for ReceivePipeImpl<F> {
    type Item = T;
    type IntoIter = IntoIter<T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T, F: FnMut() -> T> From<ReceivePipeImpl<F>> for IntoIter<T, F> {
    fn from(value: ReceivePipeImpl<F>) -> Self {
        Self(value)
    }
}
