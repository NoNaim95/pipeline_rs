pub use iterator::*;

mod iterator;
pub mod transformer;

pub trait ReceivePipe<T>: ReceivePipeMut<T> {
    fn recv(&self) -> T;
}

pub trait ReceivePipeMut<T> {
    fn recv_mut(&mut self) -> T;
}

impl<T, F: Fn() -> T> ReceivePipe<T> for F {
    fn recv(&self) -> T {
        (self)()
    }
}

impl<T, F: FnMut() -> T> ReceivePipeMut<T> for F {
    fn recv_mut(&mut self) -> T {
        (self)()
    }
}

pub trait SendPipe<T>: SendPipeMut<T> {
    fn send(&self, t: T);
}

pub trait SendPipeMut<T> {
    fn send_mut(&mut self, t: T);
}

impl<T, F: Fn(T)> SendPipe<T> for F {
    fn send(&self, t: T) {
        (self)(t)
    }
}

impl<T, F: FnMut(T)> SendPipeMut<T> for F {
    fn send_mut(&mut self, t: T) {
        (self)(t)
    }
}
