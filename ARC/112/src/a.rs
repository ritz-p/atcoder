use proconio::input;
fn main() {
    input! {
        t: usize,
        lr: [(usize,usize);t]
    };
    for (l, r) in &lr {
        if *l * 2 > *r {
            println!("0");
            continue;
        }
        let p = r - l * 2 + 1;
        let res = p * (p + 1) / 2;
        println!("{}", res);
    }
}
