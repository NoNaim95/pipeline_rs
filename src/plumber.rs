use crate::pipes::transformer_pipe::*;
use crate::pipes::*;

pub struct Plumber<P> {
    pipe: P,
}

impl<P> Plumber<P> {
    pub fn new(pipe: P) -> Self {
        Self { pipe }
    }

    pub fn complete(self) -> P {
        self.pipe
    }
}

impl<P> From<P> for Plumber<P> {
    fn from(pipe: P) -> Self {
        Self::new(pipe)
    }
}

pub trait SendPlumberMut<To, P> {
    fn with_transformer_mut<From, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: FnMut(From) -> To,
        Transformer<From, To, F, P>: SendPipeMut<From>;
}

impl<To, P: SendPipeMut<To>> SendPlumberMut<To, P> for Plumber<P> {
    fn with_transformer_mut<From, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: FnMut(From) -> To,
        Transformer<From, To, F, P>: SendPipeMut<From>,
    {
        let transformer = Transformer::new(transform, self.pipe);
        Plumber::new(transformer)
    }
}

pub trait SendPlumber<To, P> {
    fn with_transformer<From, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: Fn(From) -> To,
        Transformer<From, To, F, P>: SendPipe<From>;
}

impl<To, P: SendPipe<To>> SendPlumber<To, P> for Plumber<P> {
    fn with_transformer<From, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: Fn(From) -> To,
        Transformer<From, To, F, P>: SendPipe<From>,
    {
        let transformer = Transformer::new(transform, self.pipe);
        Plumber::new(transformer)
    }
}

pub trait ReceivePlumberMut<From, P> {
    fn with_transformer_mut<To, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: FnMut(From) -> To,
        Transformer<From, To, F, P>: ReceivePipeMut<To>;
}

impl<From, P: ReceivePipeMut<From>> ReceivePlumberMut<From, P> for Plumber<P> {
    fn with_transformer_mut<To, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: FnMut(From) -> To,
        Transformer<From, To, F, P>: ReceivePipeMut<To>,
    {
        let transformer = Transformer::new(transform, self.pipe);
        Plumber::new(transformer)
    }
}

pub trait ReceivePlumber<From, P> {
    fn with_transformer<To, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: Fn(From) -> To,
        Transformer<From, To, F, P>: ReceivePipe<To>;
}

impl<From, P: ReceivePipe<From>> ReceivePlumber<From, P> for Plumber<P> {
    fn with_transformer<To, F>(self, transform: F) -> Plumber<Transformer<From, To, F, P>>
    where
        F: Fn(From) -> To,
        Transformer<From, To, F, P>: ReceivePipe<To>,
    {
        let transformer = Transformer::new(transform, self.pipe);
        Plumber::new(transformer)
    }
}
