use proconio::input;
fn main() {
    input! {
        n: usize,
        ss: [String;n]
    };
    let mut res = 0;
    for s in ss {
        if s == "Takahashi" {
            res += 1;
        }
    }

    println!("{}", res);
}
