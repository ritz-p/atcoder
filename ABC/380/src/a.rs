use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;

    for c in s {
        match c {
            '1' => {
                one += 1;
            }
            '2' => {
                two += 1;
            }
            '3' => {
                three += 1;
            }
            _ => {}
        }
    }
    if one == 1 && two == 2 && three == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
