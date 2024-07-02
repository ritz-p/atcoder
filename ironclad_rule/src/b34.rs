use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize;n]
    };
    let xy = x + y;
    let mut res = 0;
    for i in a {
        match i % xy {
            0 => {
                res ^= 0;
            }
            1 => {
                res ^= 0;
            }
            2 => {
                res ^= 1;
            }
            3 => {
                res ^= 1;
            }
            4 => {
                res ^= 2;
            }
            _ => {}
        }
    }
    if res != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
