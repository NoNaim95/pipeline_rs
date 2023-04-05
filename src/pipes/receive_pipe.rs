use super::transformer_pipe::TransformerPipe;

pub struct ReceivePipe<T, F: FnMut() -> T>(F);
impl<T, F: FnMut() -> T> ReceivePipe<T, F> {
    pub fn new(f: F) -> Self {
        ReceivePipe(f)
    }

    pub fn recv(&mut self) -> T {
        self.0()
    }

    pub fn append_transformer<U2, F2: FnMut(T) -> U2>(
        mut self,
        t: TransformerPipe<T, U2, F2>,
    ) -> ReceivePipe<U2, impl FnMut() -> U2> {
        let mut inner = t.into_inner();
        ReceivePipe::new(move || inner(self.recv()))
    }

    pub fn iter(&mut self) -> Iter<T, F> {
        Iter(self)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: FnMut() -> T> From<F> for ReceivePipe<T, F> {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

impl<T, F: FnMut() -> Option<T>> ReceivePipe<Option<T>, F> {
    pub fn try_iter(&mut self) -> TryIter<T, F> {
        TryIter(self)
    }

    pub fn into_try_iter(self) -> IntoTryIter<T, F> {
        IntoTryIter(self)
    }
}

pub struct TryIter<'a, T, F: FnMut() -> Option<T>>(pub &'a mut ReceivePipe<Option<T>, F>);

impl<'a, T, F: FnMut() -> Option<T>> Iterator for TryIter<'a, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv()
    }
}

pub struct IntoTryIter<T, F: FnMut() -> Option<T>>(pub ReceivePipe<Option<T>, F>);

impl<T, F: FnMut() -> Option<T>> Iterator for IntoTryIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv()
    }
}

pub struct Iter<'a, T, F: FnMut() -> T>(pub &'a mut ReceivePipe<T, F>);

impl<'a, T, F: FnMut() -> T> Iterator for Iter<'a, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.recv())
    }
}

pub struct IntoIter<T, F: FnMut() -> T>(pub ReceivePipe<T, F>);

impl<T, F: FnMut() -> T> Iterator for IntoIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.recv())
    }
}

impl<T, F: FnMut() -> T> IntoIterator for ReceivePipe<T, F> {
    type Item = T;
    type IntoIter = IntoIter<T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T, F: FnMut() -> T> From<ReceivePipe<T, F>> for IntoIter<T, F> {
    fn from(value: ReceivePipe<T, F>) -> Self {
        Self(value)
    }
}
