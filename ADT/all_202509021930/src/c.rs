use proconio::input;
fn main() {
    input! {
        n: usize,
        mut sc: [(String,usize);n]
    };
    let sum: usize = sc.iter().map(|(_, c)| c).sum();
    sc.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{}", sc[sum % n].0);
}
