use pipeline_rs::pipes::receive_pipe::ReceivePipe;
use std::thread::sleep_ms;

struct Client<I: Iterator> {
    messages: I,
}

impl<I: Iterator> Client<I> {
    pub fn new(i: I) -> Self {
        Self { messages: i }
    }

    pub fn handle_messages(mut self, mut message_handler: impl FnMut(&I::Item)) {
        for msg in self.messages {
            message_handler(&msg)
        }
    }
}

fn main() {
    let mut pipe = ReceivePipe::new(|| 10).append_transformer(|x| {
        sleep_ms(50);
        x * 2
    });

    let mut client = Client::new(pipe.into_iter());
    client.handle_messages(|x| println!("The number we received was: {}", x));
}
