use proconio::input;

fn main() {
    input! {
        t: usize,
    };
    let res = weird(weird(weird(t) + t) + weird(weird(t)));

    println!("{}", res);
}

fn weird(t: usize) -> usize {
    return (2 + t) * t + 3;
}
