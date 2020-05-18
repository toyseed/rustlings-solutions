// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn _current_favorite_color() -> &'static str {
    "blue"
}
fn current_favorite_color() -> String {
    String::from("blue")
}
