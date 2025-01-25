use proconio::input;

fn main() {
    input! {
        a: [usize;5]
    };
    let mut ac = a.clone();
    ac.sort();
    for i in 0..4 {
        let mut cur = a.clone();
        let tmp = cur[i];
        cur[i] = cur[i + 1];
        cur[i + 1] = tmp;
        if cur == ac {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
