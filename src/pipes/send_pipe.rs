use crate::pipes::{SendPipe, SendPipeMut};

pub struct SendPipeImpl<T, F: FnMut(T)>(F, std::marker::PhantomData<T>);

impl<T, F: FnMut(T)> SendPipeImpl<T, F> {
    pub fn new(f: F) -> Self {
        SendPipeImpl(f, Default::default())
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: FnMut(T)> SendPipeMut<T> for SendPipeImpl<T, F> {
    fn send(&mut self, t: T) {
        self.0(t)
    }
}

impl<T, F: Fn(T)> SendPipe<T> for SendPipeImpl<T, F> {
    fn send(&self, t: T) {
        self.0(t)
    }
}
