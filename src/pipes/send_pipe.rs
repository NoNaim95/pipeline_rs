use std::marker::PhantomData;

pub struct SendPipe<T, F: FnMut(T)>(F, PhantomData<T>);
impl<T, F: FnMut(T)> SendPipe<T, F> {
    pub fn new(f: F) -> Self {
        SendPipe {
            0: f,
            1: PhantomData,
        }
    }

    pub fn send(&mut self, t: T) {
        self.0(t)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: FnMut(T)> From<F> for SendPipe<T, F> {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}
