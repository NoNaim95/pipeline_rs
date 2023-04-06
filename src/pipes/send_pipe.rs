use crate::pipes::{SendPipe, SendPipeMut};

pub struct SendPipeImpl<F>(F);

impl<F> SendPipeImpl<F> {
    pub fn new(f: F) -> Self {
        SendPipeImpl(f)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: FnMut(T)> SendPipeMut<T> for SendPipeImpl<F> {
    fn send(&mut self, t: T) {
        self.0(t)
    }
}

impl<T, F: Fn(T)> SendPipe<T> for SendPipeImpl<F> {
    fn send(&self, t: T) {
        self.0(t)
    }
}
