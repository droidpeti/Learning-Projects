fn main() {
    let text = String::from("Csibész");
    let len = calculate_length(&text);
    println!("The length of {text} is {len}");
}

fn calculate_length(text: &String) -> usize {
    text.len()
}
