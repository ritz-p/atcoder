use proconio::input;

fn main() {
    input! {
        mut xy: f64
    };

    xy *= 10.0;
    if xy % 10.0 <= 2.0 {
        println!("{:.0}-", (xy / 10.0).floor());
    } else if xy % 10.0 >= 3.0 && xy % 10.0 <= 6.0 {
        println!("{:.0}", (xy / 10.0).floor());
    } else {
        println!("{:.0}+", (xy / 10.0).floor());
    }
}
