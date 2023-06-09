use super::send_pipe::SendPipe;
use std::marker::PhantomData;

pub struct TransformerPipe<T, U, F: FnMut(T) -> U>(F, PhantomData<T>, PhantomData<U>);
impl<T, U, F: FnMut(T) -> U> TransformerPipe<T, U, F> {
    pub fn new(f: F) -> Self {
        TransformerPipe {
            0: f,
            1: PhantomData,
            2: PhantomData,
        }
    }

    pub fn transform(&mut self, x: T) -> U {
        self.0(x)
    }

    pub fn append_transformer<U2, F2: FnMut(U) -> U2>(
        mut self,
        mut t: TransformerPipe<U, U2, F2>,
    ) -> TransformerPipe<T, U2, impl FnMut(T) -> U2> {
        let mut inner = t.into_inner();
        TransformerPipe::new(move |x| inner(self.transform(x)))
    }

    pub fn append_consumer<F2: FnMut(U)>(mut self, mut t: SendPipe<U,F2>) -> SendPipe<T, impl FnMut(T)> {
        let mut inner = t.into_inner();
        let closure = move |x| inner(self.transform(x));
        SendPipe::new(closure)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, U, F: FnMut(T) -> U> From<F> for TransformerPipe<T, U, F> {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}
