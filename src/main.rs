fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let message = greet("Zed + WSL + Rust");
    println!("{}", message);
}
