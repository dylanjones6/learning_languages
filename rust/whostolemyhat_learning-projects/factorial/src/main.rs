// Make a factorial calculator
// input 0 => 1
// input 1 => 1
// input 2 => 2  = 1 * 2
// input 3 => 6  = 1 * 2 * 3
// input 4 => 24 = 1 * 2 * 3 * 4

use std::io;

fn factorial(num: u64) -> u64 {
    // if num <= 1 {
    //     return 1;
    // }

    // num * factorial(num - 1)
    if num > 1 {
        // return (num * factorial(num - 1))
        num * factorial(num - 1)
    }
    else {
        1
    }
}


fn main() {
    println!{"I want the factorial of: "};

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");
    let num = input_text.trim().parse::<u64>().expect("That's not a number");

    println!("{}", factorial(num));

    // let fact_arr = [u32, num + ]
}
