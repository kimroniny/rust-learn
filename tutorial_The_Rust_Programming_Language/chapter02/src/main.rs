fn main() {
    println!("pls input a number");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap_or_default();
    match input.strip_suffix("\n") {
        Some(x) => {input=x.to_string()},
        None => {}
    }
    println!("your input is: {input}");
}
