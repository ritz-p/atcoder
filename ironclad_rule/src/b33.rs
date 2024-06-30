use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        ab: [(usize,usize);n]
    };
    let mut xor = 0;

    for (a, b) in ab {
        xor = xor ^ (a - 1);
        xor = xor ^ (b - 1);
    }

    if xor != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
