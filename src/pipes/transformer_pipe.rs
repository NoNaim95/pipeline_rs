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
        mut t: F2,
    ) -> TransformerPipe<T, U2, impl FnMut(T) -> U2> {
        let closure = move |x| t(self.transform(x));
        TransformerPipe::new(closure)
    }

    pub fn append_consumer<F2: FnMut(U)>(mut self, mut t: F2) -> SendPipe<T, impl FnMut(T)> {
        let closure = move |x| t(self.transform(x));
        SendPipe::new(closure)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}
