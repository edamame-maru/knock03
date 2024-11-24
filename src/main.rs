use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    let num: i32 = buffer.trim().parse().unwrap();

    println!("your number is: {}", num);
}
