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
