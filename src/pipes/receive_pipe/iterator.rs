use crate::pipes::ReceivePipeMut;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Rov: Reference or value; holds either a reference or a value of type T.
enum Rov<'a, T> {
    Ref(&'a mut T),
    Val(T),
}

impl<T> From<T> for Rov<'_, T> {
    fn from(t: T) -> Self {
        Rov::Val(t)
    }
}

impl<'a, T> From<&'a mut T> for Rov<'a, T> {
    fn from(t: &'a mut T) -> Self {
        Rov::Ref(t)
    }
}

impl<T> Deref for Rov<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Rov::Ref(r) => r,
            Rov::Val(v) => v,
        }
    }
}

impl<T> DerefMut for Rov<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Rov::Ref(r) => r,
            Rov::Val(v) => v,
        }
    }
}

pub struct PipeIterator<'a, T, P: ReceivePipeMut<T>>(Rov<'a, P>, PhantomData<T>);

impl<T, P: ReceivePipeMut<Option<T>>> Iterator for PipeIterator<'_, Option<T>, P> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv_mut()
    }
}

pub trait PipeIter<T, P: ReceivePipeMut<T>> {
    fn iter(&mut self) -> PipeIterator<T, P>;
}

impl<T, P: ReceivePipeMut<T>> PipeIter<T, P> for P {
    fn iter(&mut self) -> PipeIterator<T, P> {
        PipeIterator(self.into(), PhantomData)
    }
}

pub trait IntoPipeIter<'a, T, P: ReceivePipeMut<T>> {
    fn into_iter(self) -> PipeIterator<'a, T, P>;
}

impl<'a, T, P: ReceivePipeMut<T>> IntoPipeIter<'a, T, P> for P {
    fn into_iter(self) -> PipeIterator<'a, T, P> {
        PipeIterator(self.into(), PhantomData)
    }
}
