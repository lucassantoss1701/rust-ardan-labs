use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command{
    Run(Job),
    Quit
}
fn hi_there(){
    println!("Hi there!")
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();
    let handle = std::thread::spawn(move ||{
       while let Ok(job) = rx.recv(){
           job();
       }
    });

    let job = || println!("Hello from a closure");
    let job2 = || {
        for i in 0..10{
            println!("{i}");
        }
    };

    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(|| println!("I'm in a box"))).unwrap();

    let (tx, rx) = mpsc::channel::<Command>();
    let handle = std::thread::spawn(move ||{
        while let Ok(command) = rx.recv(){
            match command {
                Command::Run(job) => job(),
                Command::Quit => break
            }
        }
    });

    tx.send(Command::Run(Box::new(|| println!("I'm in a box from command")))).unwrap();
    tx.send(Command::Quit).unwrap();

    handle.join().unwrap();
}
