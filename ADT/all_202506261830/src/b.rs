use proconio::input;
fn main() {
    input! {
        s: [usize;8]
    };

    for i in 0..8 {
        if s[i] % 25 != 0 {
            println!("No");
            return;
        }
        if s[i] < 100 || s[i] > 675 {
            println!("No");
            return;
        }
        if i < 7 {
            if s[i] > s[i + 1] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
