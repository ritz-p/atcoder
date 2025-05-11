use proconio::input;
fn main() {
    input! {
        r: usize,
        x: usize,
    };
    match x {
        1 => {
            if r >= 1600 && r <= 2999 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        2 => {
            if r >= 1200 && r <= 2399 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        _ => {}
    }
}
