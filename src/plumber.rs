use crate::pipes::transformer_pipe::*;

/// Connects pipes
pub struct Plumber<P> {
    pipe: P,
}

impl<P> Plumber<P> {
    pub fn new(pipe: P) -> Self {
        Self { pipe }
    }

    pub fn with_transformer<From, To, F>(self, t: F) -> Plumber<Transformer<From, To, F, P>> {
        let transformer = Transformer::new(t, self.pipe);
        Plumber::new(transformer)
    }

    pub fn build(self) -> P {
        self.pipe
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipes::{ReceivePipe, ReceivePipeMut, SendPipe, SendPipeMut};

    #[test]
    fn test_send() {
        let p = Plumber::new(|t: i32| assert_eq!(t, 2))
            .with_transformer(|t: i32| t + 1)
            .build();
        p.send(1);
    }

    #[test]
    fn test_send_mut() {
        let mut i = 0;
        let mut p = Plumber::new(|t: i32| i += t)
            .with_transformer(|t: i32| t + 1)
            .build();
        p.send_mut(1);
        assert_eq!(i, 2);
    }

    #[test]
    fn test_recv() {
        let p = Plumber::new(|| 1).with_transformer(|t: i32| t + 1).build();
        assert_eq!(p.recv(), 2);
    }

    #[test]
    fn test_recv_mut() {
        let mut i = 1;
        let mut p = Plumber::new(|| 1)
            .with_transformer(|t: i32| {
                i += t;
                i
            })
            .build();
        assert_eq!(p.recv_mut(), 2);
    }
}
