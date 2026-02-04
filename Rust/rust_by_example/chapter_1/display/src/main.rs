use std::fmt;

struct Complex{
    x: f64,
    y: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut symbol : &str = "+";
        if self.y <= 0.0 {
            symbol = "";
        }
        write!(f, "{} {symbol}{}i", self.x, self.y)
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

fn main() {
    let comp = Complex { x: 3.3, y: 7.2 };

    println!("Display: {}", comp);
    println!("Debug: {:?}", comp);

    let comp2 = Complex { x: 4.7, y: -2.3};
    println!("Display: {}", comp2);
    println!("Debug: {:?}", comp2);
}
