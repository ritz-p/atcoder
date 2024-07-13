use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    };

    match c.as_str() {
        "Red" => {
            println!("{}", g.min(b));
        }
        "Green" => {
            println!("{}", r.min(b));
        }
        "Blue" => {
            println!("{}", r.min(g));
        }
        _ => {}
    }
}
