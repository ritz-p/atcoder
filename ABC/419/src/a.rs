use proconio::input;
fn main() {
    input! {
        s: String
    };
    if s == "red".to_string() {
        println!("SSS");
    } else if s == "blue".to_string() {
        println!("FFF");
    } else if s == "green".to_string() {
        println!("MMM");
    } else {
        println!("Unknown");
    }
}
