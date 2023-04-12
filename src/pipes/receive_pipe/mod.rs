use crate::pipes::{ReceivePipe, ReceivePipeMut};
pub use iterator::*;

mod iterator;

pub struct ReceivePipeImpl<F>(F);

impl<T, F: FnMut() -> T> ReceivePipeImpl<F> {
    pub fn new(f: F) -> Self {
        ReceivePipeImpl(f)
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
