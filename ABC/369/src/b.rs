use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut res = 0;
    let mut r = 101;
    let mut l = 101;
    for _i in 0..n {
        input! {
            a: isize,
            s: char
        };
        match s {
            'L' => {
                if l == 101 {
                    l = a;
                } else {
                    res += (l - a).abs();
                    l = a;
                }
            }
            'R' => {
                if r == 101 {
                    r = a;
                } else {
                    res += (r - a).abs();
                    r = a;
                }
            }
            _ => {}
        }
    }
    println!("{}", res);
}
