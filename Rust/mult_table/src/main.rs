use std::io;
use std::io::Write;
use std::process;

fn main() {
    println!("Welcome to the Multiplication Table program!");
    let rows = read_int("rows");
    let cols = read_int("cols");

    let mut i = 1;
    while i <= rows{
        let mut j = 1;
        while j <= cols{
            print!("{} ", i*j);
            io::stdout().flush().unwrap();
            j += 1;
        }
        println!();
        i += 1;
    }
}

fn read_int(name: &str) -> u32 {
    println!("Please enter {name}: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read {name}!");

    let num : u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(1),
    };
    num
}
