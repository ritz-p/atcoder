use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let res = 32_usize.pow(a as u32 - b as u32);
    println!("{}", res);
}
