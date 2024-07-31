use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
        mut x: usize,
    };
    let sum = a.iter().sum::<usize>();
    let div = x / sum;
    x %= sum;
    for (index, e) in a.iter().enumerate() {
        if x >= *e {
            x -= e;
        } else {
            println!("{}", div * n + index + 1);
            return;
        }
    }
}
