// This file exists to send things into mpsc.
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Wrapper<'a> {
    input: &'a str,
}

// try to send something with a struct that has a lifetime
#[derive(Clone)]
struct WWrapper<'a> {
    w: Wrapper<'a>,
}

// main function
fn main() {
    // create a mspc for the struct
    let (tx, rx): (Sender<WWrapper>, Receiver<WWrapper>) = mpsc::channel();

    // start the send loop
    thread::spawn(move || {
        send_loop(tx);
    });

    // start the read loop
    read_loop(rx);
}

// function to send stuff into queue often
fn send_loop(tx: Sender<WWrapper>) {
    let input = Wrapper {
        input: "message",
    };

    let input_WWrapper = WWrapper {
        w: input,
    };

    let five_seconds = Duration::new(5, 0);

    // start the send loop
    loop {
        // Make a struct
        let pass_in = input_WWrapper.clone();

        // send the struct into the channel
        tx.send(pass_in).unwrap();

        thread::sleep(five_seconds);
    }
}

// function to continuously read stuff from
fn read_loop(rx: Receiver<WWrapper>) {
    // This is an infinite loop.
    for received in rx {
        println!("Got: {}", received.w.input);
    }
}
