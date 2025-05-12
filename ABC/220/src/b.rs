use proconio::input;

fn main() {
    input! {
        k: usize,
        a: String,
        b: String,
    };
    let res =
        usize::from_str_radix(&a, k as u32).unwrap() * usize::from_str_radix(&b, k as u32).unwrap();
    println!("{}", res);
}
