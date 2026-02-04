fn main() {
    let text : String = String::from("Hello!");
    let text_ref : &str = &text;
    write(&text);
    write(text_ref);
}

fn write(text : &str) {
    println!("{text}");
}
