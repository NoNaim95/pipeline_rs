use crate::pipes::SendPipe;
use std::marker::PhantomData;

pub struct SendPipeImpl<T, F: FnMut(T)>(F, PhantomData<T>);

impl<T, F: FnMut(T)> SendPipeImpl<T, F> {
    pub fn new(f: F) -> Self {
        SendPipeImpl(f, PhantomData)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: FnMut(T)> SendPipe<T> for SendPipeImpl<T, F> {
    fn send(&mut self, t: T) {
        self.0(t)
    }
}

impl<T, F: FnMut(T)> From<F> for SendPipeImpl<T, F> {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}
