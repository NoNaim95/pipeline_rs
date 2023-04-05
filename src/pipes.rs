pub mod receive_pipe;
pub mod send_pipe;
pub mod transformer_pipe;

pub trait ReceivePipe<T> {
    fn recv(&mut self) -> T;
}

pub trait SendPipe<T> {
    fn send(&mut self, t: T);
}

impl<T, F: Fn() -> T> ReceivePipe<T> for F {
    fn recv(&mut self) -> T {
        (self)()
    }
}

impl<T, F: Fn(T)> SendPipe<T> for F {
    fn send(&mut self, t: T) {
        (self)(t)
    }
}
