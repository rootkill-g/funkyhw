use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

fn main() {
    let (tx1, rx1) = channel();
    let tx = Sender::clone(&tx1);
    let (tx2, rx2) = channel();

    let thread_1 = thread::spawn(move || loop {
        if rx1.recv().unwrap() == 1 {
            println!("Hello");
            tx2.send(2).unwrap();
        }
    });

    let thread_2 = thread::spawn(move || loop {
        if rx2.recv().unwrap() == 2 {
            println!("World");
            tx1.send(1).unwrap();
        }
    });

    tx.send(1).unwrap();

    thread_1.join().ok();
    thread_2.join().ok();
}
