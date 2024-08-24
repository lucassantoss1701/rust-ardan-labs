fn main() {
    // run cargo fmt
    // cargo watch -x 'fmt' to follow
    println!("Hello, world!");for i in 0..21 { println!(".)");}

    #[rustfmt::skip] // will not be formatted
    mod section{
        const N: i32 = 1;
    }
}
