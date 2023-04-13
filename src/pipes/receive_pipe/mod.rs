use crate::pipes::{ReceivePipe, ReceivePipeMut};
pub use iterator::*;

mod iterator;

pub struct ReceivePipeImpl<F>(F);

impl<F> ReceivePipeImpl<F> {
    pub fn new(f: F) -> Self {
        ReceivePipeImpl(f)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: FnMut() -> T> ReceivePipeMut<T> for ReceivePipeImpl<F> {
    fn recv_mut(&mut self) -> T {
        self.0()
    }
}

impl<T, F: Fn() -> T> ReceivePipe<T> for ReceivePipeImpl<F> {
    fn recv(&self) -> T {
        self.0()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recv() {
        assert_eq!(ReceivePipeImpl::new(|| 1).recv(), 1);
    }

    #[test]
    fn test_recv_mut() {
        let mut i = 0;
        assert_eq!(
            ReceivePipeImpl::new(|| {
                i += 1;
                i
            })
            .recv_mut(),
            1
        );
    }
}
