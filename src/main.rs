use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
};
use std::time::Duration;
use std::thread;

fn main() {
    startup_messages();
    let hoge = check_no_error(input());
    println!("{}", hoge);
}

fn startup_messages() {
    println!("3");
    thread::sleep(Duration::from_millis(1000));
    println!("2");
    thread::sleep(Duration::from_millis(1000));
    println!("1");
    thread::sleep(Duration::from_millis(1000));
    println!("Start!!");
}

fn input() -> Result<String, std::io::Error> {
    let mut line = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()?{
        match code {
            KeyCode::Char(c) => line.push(c),
            KeyCode::Enter => break,
            _ => {},
        }
    }
    Ok(line)
}

fn check_no_error(result: Result<String, std::io::Error>) -> String{
    match result {
        Ok(line) => return String::from(line),
        Err(_) => return String::from("hoge"),
    }
}
