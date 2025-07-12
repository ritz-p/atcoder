use proconio::input;
fn main() {
    input! {
        n: usize,
        cl: [(char,usize);n]
    };
    if cl.iter().any(|(_c, l)| *l > 100) {
        println!("Too Long");
        return;
    }
    let sum = cl.iter().map(|(_c, l)| l).sum::<usize>();
    if sum > 100 {
        println!("Too Long");
        return;
    }
    for (c, l) in cl {
        for _i in 0..l {
            print!("{}", c);
        }
    }
    println!();
}
