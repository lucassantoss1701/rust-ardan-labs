use std::sync::mpsc;
use std::{thread};

enum Command {
    SayHello, Quit
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move ||{
        while let Ok(command) = rx.recv(){
            match command {
                Command::SayHello => println!("Hello"),
                Command::Quit => {
                    println!("Quitting");
                    break;
                }
            }
        }
    });


    for _ in 0..10{
        tx.send(Command::SayHello).unwrap();
    }

    thread::sleep(std::time::Duration::from_secs(1));
    println!("Sending quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();

}
