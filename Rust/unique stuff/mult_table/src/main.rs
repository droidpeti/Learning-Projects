use std::io;

fn main() {
    println!("Welcome to the Multiplication Table program!");
    let rows = read_int("rows");
    let cols = read_int("cols");
    let cell_width = (rows*cols).to_string().len() + 1;

    for i in 1..=rows {
        for j in 1..=cols {
            print!("{:cell_width$}", i * j); 
        }
        println!();
    }
}

fn read_int(name: &str) -> u32 {
    loop {
        println!("Please enter {name}: ");
        let mut num = String::new();
        
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read input");

        match num.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Invalid input, please enter a positive number."),
        }
    }
}
