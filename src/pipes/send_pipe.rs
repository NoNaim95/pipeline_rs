use crate::pipes::{SendPipe, SendPipeMut};

pub struct SendPipeImpl<F>(F);

impl<F> SendPipeImpl<F> {
    pub fn new(f: F) -> Self {
        SendPipeImpl(f)
    }

    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F: FnMut(T)> SendPipeMut<T> for SendPipeImpl<F> {
    fn send_mut(&mut self, t: T) {
        self.0(t)
    }
}

impl<T, F: Fn(T)> SendPipe<T> for SendPipeImpl<F> {
    fn send(&self, t: T) {
        self.0(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        SendPipeImpl::new(|t: i32| {
            assert_eq!(t, 1);
        })
        .send(1);
    }

    #[test]
    fn test_send_mut() {
        let mut i = 0;
        SendPipeImpl::new(|t: i32| {
            i += t;
        })
        .send_mut(1);
        assert_eq!(i, 1);
    }
}
