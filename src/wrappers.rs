/*use crate::pipes::{ReceivePipe, SendPipe};

pub trait Source<T> {
    fn take(&mut self) -> T;
}

impl<T, U: ReceivePipe<T>> Source<T> for U {
    fn take(&mut self) -> T {
        self.recv_mut()
    }
}

pub trait Destination<T> {
    fn put(&mut self, t: T);
}

impl<T, U: SendPipe<T>> Destination<T> for U {
    fn put(&mut self, t: T) {
        self.send_mut(t)
    }
}
*/
