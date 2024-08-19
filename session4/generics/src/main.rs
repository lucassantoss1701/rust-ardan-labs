use std::fmt::Debug;

fn just_print_it<T: ToString>(x: T){
    println!("{}", x.to_string());
}

fn just_print_it_with_where<T, U>(x: T, y: U)
where T: ToString + Debug, U: ToString,
{
    println!("{}", x.to_string());
    println!("{}", y.to_string());

}

fn main() {
    just_print_it("Hello");
    just_print_it(5);

    just_print_it_with_where("Hello", 5);
    just_print_it_with_where(5, "12");
}
