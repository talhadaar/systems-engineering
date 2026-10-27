use std::{
    sync::atomic::{AtomicBool, Ordering::Relaxed},
    thread,
};

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // bg worker
    let bg_worker = thread::spawn(|| {
        // keep working while not stopped
        while !STOP.load(Relaxed) {
            // some work
        }
    });

    // main thread
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            _ => println!("unknown cmd"),
        }
    }

    STOP.store(true, Relaxed);

    bg_worker.join().unwrap();
}
