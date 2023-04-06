use pipeline_rs::pipes::receive_pipe::ReceivePipeImpl;
use pipeline_rs::pipes::transformer_pipe::Transformer;
use pipeline_rs::pipes::*;
use std::{sync::mpsc::channel, thread::sleep, time::Duration};

struct Client<I: Iterator> {
    messages: I,
}

impl<I: Iterator> Client<I> {
    pub fn new(i: I) -> Self {
        Self { messages: i }
    }

    pub fn handle_messages(&mut self, mut message_handler: impl FnMut(&I::Item)) {
        let mut n = 0;
        for msg in &mut self.messages {
            message_handler(&msg);
            n += 1;
        }
        println!("There were {} messages in the buffer", n);
    }
}

fn main() {
    let (s, r) = channel();
    let closure = move |x| {
        sleep(Duration::from_millis(300));
        s.send(x).unwrap();
    };

    let send_pipe1 = Transformer::new(|x| x + 100, closure);

    let mut send_pipe = Transformer::new(|x| x + 100, send_pipe1);

    std::thread::spawn(move || loop {
        send_pipe.send(10);
    });
    sleep(Duration::from_millis(970));

    let pipe = ReceivePipeImpl::new(|| r.try_recv().ok());
    let try_iter = pipe.into_try_iter().map(|x| x * 3);
    let mut client = Client::new(try_iter);

    client.handle_messages(|msg| println!("[HANDLER]: msg: {}", msg));
    sleep(Duration::from_millis(3203));
    client.handle_messages(|msg| println!("[HANDLER]: msg: {}", msg));
}
