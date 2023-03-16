use std::time::Duration;
use std::thread;

fn main() {
    println!("0 second");
    thread::sleep(Duration::from_millis(1000));
    println!("1 second");
    thread::sleep(Duration::from_millis(1000));
    println!("2 second");
}
