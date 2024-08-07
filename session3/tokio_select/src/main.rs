use std::time::Duration;

async fn do_work(){
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32){
    
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_work() => println!("do_work finished fisrt"),
        _ = timeout(1.0) => println!("timeout finished first"),
    };
}
