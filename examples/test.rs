use pipeline_rs::pipes::{
    receive_pipe::ReceivePipe, send_pipe::SendPipe, transformer_pipe::TransformerPipe
};
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
    let (s, r) = channel::<i32>();
    let closure = move |x| {
        sleep(Duration::from_millis(300));
        s.send(x).unwrap();
    };

    let send_pipe1 = TransformerPipe::new(|x| x + 100).append_consumer(closure.into());

    let mut send_pipe = TransformerPipe::new(|x| x + 100).append_consumer(send_pipe1);

    std::thread::spawn(move || loop {
        send_pipe.send(10);
    });
    sleep(Duration::from_millis(970));

    let try_iter = ReceivePipe::new(|| r.try_recv().ok())
        .into_try_iter()
        .map(|x| {x * 3});
    let mut client = Client::new(try_iter);

    client.handle_messages(|msg| println!("[HANDLER]: msg: {}", msg));
    sleep(Duration::from_millis(3203));
    client.handle_messages(|msg| println!("[HANDLER]: msg: {}", msg));
}
