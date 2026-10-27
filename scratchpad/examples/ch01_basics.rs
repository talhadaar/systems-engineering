use std::thread;

fn main() {
    let t = thread::spawn(|| println!("hello from {:?}", thread::current().id()));
    t.join().unwrap();
}
