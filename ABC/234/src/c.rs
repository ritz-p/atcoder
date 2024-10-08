use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let s = format!("{:b}", n).replace("1", "2");
    println!("{}", s);
}
