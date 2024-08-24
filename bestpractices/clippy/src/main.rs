fn main() {

    // run cargo clippy lint
    let numbers = (0..100).collect::<Vec<i32>>();
    for i in 0.. numbers.len(){
        println!("{}",numbers[i])
    }
}
