// This file exists to send things into mpsc.
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

// struct to have a &str
// struct with String first, then &str

// main function
fn main() {
    // create a mspc for the struct
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // start the send loop
    thread::spawn(move || {
        send_loop(tx);
    });

    // start the read loop
    read_loop(rx);
}

// function to send stuff into queue often
fn send_loop(tx: Sender<String>) {
    let input = "message".to_string();
    let five_seconds = Duration::new(5, 0);

    // start the send loop
    loop {
        // Make a struct
        let pass_in = input.clone();

        // send the struct into the channel
        tx.send(pass_in).unwrap();

        thread::sleep(five_seconds);
    }
}

// function to continuously read stuff from
fn read_loop(rx: Receiver<String>) {
    for received in rx {
        println!("Got: {}", received);
    }
}
