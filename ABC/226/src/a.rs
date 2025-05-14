use proconio::input;

fn main() {
    input! {
        x: f64
    };
    println!("{:.0}", x.round());
}
