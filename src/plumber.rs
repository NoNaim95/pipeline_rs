use crate::pipes::transformer_pipe::*;
use crate::pipes::*;
use std::marker::PhantomData;

pub struct Plumber<T, P> {
    pipe: P,
    phantom: PhantomData<T>,
}

impl<T, P> Plumber<T, P> {
    pub fn new(pipe: P) -> Self {
        Self {
            pipe,
            phantom: Default::default(),
        }
    }
}

impl<T, P> From<P> for Plumber<T, P> {
    fn from(pipe: P) -> Self {
        Self::new(pipe)
    }
}

pub trait SendPlumber<T, P> {
    fn with_transformer<T2, F: FnMut(T2) -> T>(self, t: F) -> Plumber<T2, Transformer<T2, T, F, P>>
    where
        Transformer<T2, T, F, P>: SendPipe<T2>;
    fn complete(self) -> P;
}

pub trait ReceivePlumber<T, P> {
    fn with_transformer<T2, F: FnMut(T) -> T2>(self, t: F) -> Plumber<T2, Transformer<T, T2, F, P>>
    where
        Transformer<T, T2, F, P>: ReceivePipe<T2>;
    fn complete(self) -> P;
}

impl<T, P: SendPipe<T>> SendPlumber<T, P> for Plumber<T, P> {
    fn with_transformer<T2, F: FnMut(T2) -> T>(self, t: F) -> Plumber<T2, Transformer<T2, T, F, P>>
    where
        Transformer<T2, T, F, P>: SendPipe<T2>,
    {
        let transformer = Transformer::new(t, self.pipe);
        Plumber::new(transformer)
    }

    fn complete(self) -> P {
        self.pipe
    }
}

impl<T, P: ReceivePipe<T>> ReceivePlumber<T, P> for Plumber<T, P> {
    fn with_transformer<T2, F: FnMut(T) -> T2>(self, t: F) -> Plumber<T2, Transformer<T, T2, F, P>>
    where
        Transformer<T, T2, F, P>: ReceivePipe<T2>,
    {
        let transformer = Transformer::new(t, self.pipe);
        Plumber::new(transformer)
    }

    fn complete(self) -> P {
        self.pipe
    }
}
