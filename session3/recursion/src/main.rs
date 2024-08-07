use std::future::Future;
use std::pin::Pin;
use async_recursion::*;

#[async_recursion]
async fn fibonacci(n: u32) -> u32{
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1).await + fibonacci(n-2).await
    }
}

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

async fn call_one_of_them(n: u32) -> Pin<Box<dyn Future<Output = ()>+ Send>>{
  match n {
      1 => Box::pin(one()),
      2 => Box::pin(two()),
      _ => panic!("Invalid choice")
  }
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", fibonacci(10).await);

    let  mut future = async {
        println!("Hello World");
    };

    tokio::pin!(future);
    (&mut future).await;



    let spawned = tokio::spawn( async {
        for n in 1..=3{
            let pin_call = call_one_of_them(n).await;
            tokio::spawn(pin_call);
        }
    });

    spawned.await.unwrap();
}
