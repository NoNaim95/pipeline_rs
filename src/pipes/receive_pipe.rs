pub struct ReceivePipe<T, F: FnMut() -> T>(F);
impl<T, F: FnMut() -> T> ReceivePipe<T, F> {
    pub fn new(f: F) -> Self {
        ReceivePipe { 0: f }
    }

    pub fn recv(&mut self) -> T {
        self.0()
    }

    pub fn append_transformer<U2, F2: FnMut(T) -> U2>(
        mut self,
        mut t: F2,
    ) -> ReceivePipe<U2, impl FnMut() -> U2> {
        ReceivePipe::new(move || t(self.recv()))
    }

    pub fn iter<'a>(&'a mut self) -> Iter<T, F> {
        Iter(self)
    }

    pub fn into_inner(self) -> F {
        self.0
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
