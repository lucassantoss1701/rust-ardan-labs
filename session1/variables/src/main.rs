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


fn main() {
    let mut name = "Hello".to_string();
    greet_borrow_mut(&mut name);
    println!("{name}");

}
