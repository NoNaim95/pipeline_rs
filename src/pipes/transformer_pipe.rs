use crate::pipes::{ReceivePipe, ReceivePipeMut, SendPipe, SendPipeMut};
use std::marker::PhantomData;

pub struct Transformer<I, O, F, P> {
    transform: F,
    pipe: P,
    phantom: PhantomData<(I, O)>,
}

impl<I, O, F, P> Transformer<I, O, F, P> {
    pub fn new(transform: F, pipe: P) -> Self {
        Self {
            transform,
            pipe,
            phantom: PhantomData::default(),
        }
    }
}

impl<I, O, F: Fn(I) -> O, P: SendPipe<O>> SendPipe<I> for Transformer<I, O, F, P> {
    fn send(&self, t: I) {
        self.pipe.send((self.transform)(t));
    }
}

impl<I, O, F: Fn(I) -> O, P: ReceivePipe<I>> ReceivePipe<O> for Transformer<I, O, F, P> {
    fn recv(&self) -> O {
        (self.transform)(self.pipe.recv())
    }
}

impl<I, O, F: FnMut(I) -> O, P: SendPipeMut<O>> SendPipeMut<I> for Transformer<I, O, F, P> {
    fn send_mut(&mut self, t: I) {
        self.pipe.send_mut((self.transform)(t));
    }
}

impl<I, O, F: FnMut(I) -> O, P: ReceivePipeMut<I>> ReceivePipeMut<O> for Transformer<I, O, F, P> {
    fn recv_mut(&mut self) -> O {
        (self.transform)(self.pipe.recv_mut())
    }
}
