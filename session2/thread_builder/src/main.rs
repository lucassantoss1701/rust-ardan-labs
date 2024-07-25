use std::thread;

fn my_thread(){
    println!("Hello from a thread name {}",
        thread::current().name().unwrap()
    );
}

fn main() {
    thread::Builder::new()
        .name("Named thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap()
        .join()
        .unwrap();
}
