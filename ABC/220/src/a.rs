use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let mut index = 1;
    while c * index <= b {
        if c * index >= a {
            println!("{}", c * index);
            return;
        }
        index += 1;
    }
    println!("-1");
}
