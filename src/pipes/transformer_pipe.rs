use crate::pipes::{ReceivePipe, SendPipe};
use std::marker::PhantomData;

pub struct Transformer<I, O, F: FnMut(I) -> O, P> {
    transform: F,
    pipe: P,
    phantom: PhantomData<(I, O)>,
}

impl<I, O, F: FnMut(I) -> O, P> Transformer<I, O, F, P> {
    pub fn new(transform: F, pipe: P) -> Self {
        Self {
            transform,
            pipe,
            phantom: PhantomData::default(),
        }
    }
}

impl<I, O, F: FnMut(I) -> O, P: SendPipe<O>> SendPipe<I> for Transformer<I, O, F, P> {
    fn send(&mut self, t: I) {
        self.pipe.send((self.transform)(t));
    }
}

impl<I, O, F: FnMut(I) -> O, P: ReceivePipe<I>> ReceivePipe<O> for Transformer<I, O, F, P> {
    fn recv(&mut self) -> O {
        (self.transform)(self.pipe.recv())
    }
}

/*
pub struct SendTransformer<T, U, F: FnMut(T) -> U, O: SendPipe<U>> {
    transform: F,
    target: O,
    phantom: PhantomData<(T, U)>,
}

impl<T, U, F: FnMut(T) -> U, O: SendPipe<U>> SendTransformer<T, U, F, O> {
    pub fn new(transform: F, target: O) -> Self {
        SendTransformer {
            transform,
            target,
            phantom: PhantomData,
        }
    }
}

impl<T, U, F: FnMut(T) -> U, O: SendPipe<U>> SendPipe<T> for SendTransformer<T, U, F, O> {
    fn send(&mut self, t: T) {
        let result = (self.transform)(t);
        self.target.send(result);
    }
}

pub struct ReceiveTransformer<T, U, F: FnMut(T) -> U, I: ReceivePipe<T>> {
    transform: F,
    source: I,
    phantom: PhantomData<(T, U)>,
}

impl<T, U, F: FnMut(T) -> U, I: ReceivePipe<T>> ReceiveTransformer<T, U, F, I> {
    pub fn new(transform: F, source: I) -> Self {
        ReceiveTransformer {
            transform,
            source,
            phantom: PhantomData,
        }
    }
}

impl<T, U, F: FnMut(T) -> U, O: ReceivePipe<T>> ReceivePipe<U> for ReceiveTransformer<T, U, F, O> {
    fn recv(&mut self) -> U {
        let item = self.source.recv();
        (self.transform)(item)
    }
}*/
/*
pub struct Transformer<T, U, F: FnMutMut(T) -> U>(F, PhantomData<T>, PhantomData<U>);

impl<T, U, F: FnMutMut(T) -> U> Transformer<T, U, F> {
    pub fn new(f: F) -> Self {
        Transformer(f, PhantomData, PhantomData)
    }

    pub fn transform(&mut self, x: T) -> U {
        self.0(x)
    }

    pub fn append_transformer<U2, F2: FnMutMut(U) -> U2>(
        mut self,
        t: Transformer<U, U2, F2>,
    ) -> Transformer<T, U2, impl FnMutMut(T) -> U2> {
        let mut inner = t.into_inner();
        Transformer::new(move |x| inner(self.transform(x)))
    }

    pub fn append_consumer<F2: FnMutMut(U)>(
        mut self,
        t: SendPipeImpl<U, F2>,
    ) -> SendPipeImpl<T, impl FnMutMut(T)> {
        let mut inner = t.into_inner();
        let closure = move |x| inner(self.transform(x));
        SendPipeImpl::new(closure)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, U, F: FnMutMut(T) -> U> From<F> for Transformer<T, U, F> {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}
*/
