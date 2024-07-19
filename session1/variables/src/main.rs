fn double(n: i32) -> i32{
    n * 2
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
       return  n * 2
    }

    0
}

fn main() {
    let n = double(5);
    println!("{n}");

    let n = double_or_nothing(6);
    println!("{n}");

    let i = if n == 6{
        6
    }else{
        7
    };

    println!("{i}");
}
