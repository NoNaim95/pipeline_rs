use pipeline_rs::pipes::*;
use pipeline_rs::plumber::*;
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
    let mut i = 0;
    let closure = move |x| {
        i += 1;
        s.send(x + i).unwrap();
    };

    let t = Plumber::from_sink(|a: String| println!("{}", a))
        .with_transformer(|i: i32| i.to_string())
        .with_transformer(|(a, b)| a + b)
        .build();
    t.send((1, 2));

    let t = Plumber::from_source(|| 42)
        .with_transformer(|n| format!("Zahl: {}", n))
        .with_transformer(|s: String| s + "...")
        .build();
    println!("{}", t.recv());

    let mut send_pipe = Plumber::from_mut_sink(closure)
        .with_transformer(|x| x + 100)
        .with_transformer(|x| x + 100)
        .build();

    std::thread::spawn(move || loop {
        sleep(Duration::from_millis(300));
        send_pipe.send_mut(10);
    });
    sleep(Duration::from_millis(970));

    let try_iter = (|| r.try_recv().ok()).into_iter().map(|x| x * 3);
    let mut client = Client::new(try_iter);

    client.handle_messages(|msg| println!("[HANDLER]: msg: {}", msg));
    sleep(Duration::from_millis(3203));
    client.handle_messages(|msg| println!("[HANDLER]: msg: {}", msg));
}
