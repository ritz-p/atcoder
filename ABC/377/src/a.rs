use proconio::input;

fn main() {
    input! {
        s: String
    };
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for d in s.chars() {
        if d == 'A' {
            a += 1;
        } else if d == 'B' {
            b += 1;
        } else if d == 'C' {
            c += 1;
        }
    }
    if a == 1 && b == 1 && c == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
