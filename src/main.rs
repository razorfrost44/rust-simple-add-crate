use simple_add_crate::simple_add::*;

fn main() {
    println!("START");
    println!();

    let a = 7;
    let b = 9;
    let result = add_two(a, b);
    println!("The sum of {} and {} is {}", a, b, result);

    println!();
    println!("END");
}
