use proconio::input;
 
fn main() {
    input! {
        x1: i64,
        y1: i64,
        r1: i64,
        x2: i64,
        y2: i64,
        r2: i64,
    };
    // pow = べき乗 pow(2) 二乗
 
    let d = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
 
    if d < (r1 - r2).pow(2) {
        println!("1")
    } else if d == (r1 - r2).pow(2) {
        println!("2")
    } else if d < (r1 + r2).pow(2) {
        println!("3")
    } else if d == (r1 + r2).pow(2) {
        println!("4")
    } else {
        println!("5")
    }
}