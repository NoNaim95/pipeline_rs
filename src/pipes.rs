pub mod receive_pipe;
pub mod send_pipe;
pub mod transformer_pipe;

pub trait ReceivePipe<T>: ReceivePipeMut<T> {
    fn recv(&self) -> T;
}

pub trait SendPipe<T>: SendPipeMut<T> {
    fn send(&self, t: T);
}

pub trait ReceivePipeMut<T> {
    fn recv(&mut self) -> T;
}

pub trait SendPipeMut<T> {
    fn send(&mut self, t: T);
}

impl<T, F: FnMut() -> T> ReceivePipeMut<T> for F {
    fn recv(&mut self) -> T {
        (self)()
    }
}

impl<T, F: FnMut(T)> SendPipeMut<T> for F {
    fn send(&mut self, t: T) {
        (self)(t)
    }
}

impl<T, F: Fn() -> T> ReceivePipe<T> for F {
    fn recv(&self) -> T {
        (self)()
    }
}

impl<T, F: Fn(T)> SendPipe<T> for F {
    fn send(&self, t: T) {
        (self)(t)
    }
}
