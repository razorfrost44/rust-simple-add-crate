// Imports - lib
use simple_add_crate::simple_add::*;
// Imports - examples
pub mod apple;
use apple::*;

fn main() {
    println!("START");
    println!();

    println!("Example usage from lib");

    let a = 7;
    let b = 9;
    let result = add_two(a, b);
    println!("The sum of {} and {} is {}", a, b, result);

    println!();
    println!("Example usage from internal extra files from examples folder");
    print_apple();

    println!();
    println!("END");
}
