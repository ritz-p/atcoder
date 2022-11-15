use proconio::input;
 
fn main() {
    input! {
        n: i64,
        a_arr: [i64; n],
        b_arr: [i64; n],
    };
 
    let mut res = 0.0;
    // zip で iter を複数回す
    for (a, b) in a_arr.iter().zip(b_arr.iter()) {
        res += (*a as f64) / 3.0;
        res += 2.0 * (*b as f64) / 3.0;
    }
 
    println!("{}", res);
}