use proconio::input;

fn main() {
    input! {
        a: [usize;4]
    };
    let mut v = vec![0; 4];
    for i in a {
        v[i - 1] += 1;
    }

    let mut res = 0;
    for i in v {
        res += i / 2;
    }

    println!("{}", res);
}
