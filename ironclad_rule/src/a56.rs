use proconio::input;
fn main() {
    input! {
        _n: usize,
        q: usize,
        mut s: String,
        queries: [(usize,usize,usize,usize);q]
    };
    s = format!("-{}", s);
    for (a, b, c, d) in queries {
        if s[a..=b] == s[c..=d] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
