use proconio::input;

fn main() {
    input! {
        s: [char;3],
        t: [char;3],
    };
    let diff = (0..3).filter(|&i| s[i] != t[i]).count();

    println!("{}", if diff == 2 { "No" } else { "Yes" });
}
