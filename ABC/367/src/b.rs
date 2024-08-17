use proconio::input;

fn main() {
    input! {
        x: f64
    };
    let mut s = format!("{}", x);
    if s.contains('.') {
        s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    }

    println!("{}", s);
}
