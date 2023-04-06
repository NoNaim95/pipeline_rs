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

pub trait SendPlumber<T, P> {
    fn with_transformer<T2, F: FnMut(T2) -> T>(self, t: F) -> Plumber<Transformer<T2, T, F, P>>
    where
        Transformer<T2, T, F, P>: SendPipe<T2>;
}

pub trait ReceivePlumber<T, P> {
    fn with_transformer<T2, F: FnMut(T) -> T2>(self, t: F) -> Plumber<Transformer<T, T2, F, P>>
    where
        Transformer<T, T2, F, P>: ReceivePipe<T2>;
}

impl<T, P: SendPipe<T>> SendPlumber<T, P> for Plumber<P> {
    fn with_transformer<T2, F: FnMut(T2) -> T>(self, t: F) -> Plumber<Transformer<T2, T, F, P>>
    where
        Transformer<T2, T, F, P>: SendPipe<T2>,
    {
        let transformer = Transformer::new(t, self.pipe);
        Plumber::new(transformer)
    }
}

impl<T, P: ReceivePipe<T>> ReceivePlumber<T, P> for Plumber<P> {
    fn with_transformer<T2, F: FnMut(T) -> T2>(self, t: F) -> Plumber<Transformer<T, T2, F, P>>
    where
        Transformer<T, T2, F, P>: ReceivePipe<T2>,
    {
        let transformer = Transformer::new(t, self.pipe);
        Plumber::new(transformer)
    }
}