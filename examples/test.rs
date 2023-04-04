use pipeline_rs::pipes::{receive_pipe::ReceivePipe, send_pipe::SendPipe};
use std::{sync::mpsc::channel, thread::sleep, time::Duration};

struct Client<I: Iterator> {
    messages: I,
}

impl<I: Iterator> Client<I> {
    pub fn new(i: I) -> Self {
        Self { messages: i }
    }

    pub fn handle_messages(self, mut message_handler: impl FnMut(&I::Item)) {
        let mut n = 0;
        for msg in self.messages {
            message_handler(&msg);
            n += 1;
        }
        println!("There were {} messages in the buffer", n);
    }
}

fn main() {
    let (s, r) = channel::<i32>();

    let mut send_pipe = SendPipe::new(move |x| {
        sleep(Duration::from_millis(300));
        s.send(x).unwrap();
    });

    std::thread::spawn(move || loop {
        send_pipe.send(10);
    });
    sleep(Duration::from_millis(970));

    let mut recv_pipe = ReceivePipe::new(||{r.try_recv().ok()});
    let client = Client::new(recv_pipe.try_iter());
    client.handle_messages(|msg|{println!("[HANDLER]: msg: {}",msg)})
}
