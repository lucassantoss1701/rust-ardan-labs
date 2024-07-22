fn double(n: i32) -> i32{
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
       return  n * 2
    }

    0
}

fn greet(s: String) -> String{
    println!("Hello {s}");
    s
}

fn greet_borrow(s: &String) {
    println!("{s}");
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("Hello {s}");
}

fn read_line() -> String{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}


fn main() {
   let input = read_line();
    println!("You typed: [{input}]")

}
