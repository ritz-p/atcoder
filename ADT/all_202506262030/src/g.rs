use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut l: [usize;n]
    };

    l = l.iter().map(|e| e + 1).collect::<Vec<usize>>();
    let mut left = l.iter().max().unwrap() - 1;
    let mut right = l.iter().sum::<usize>();

    while left + 1 < right {
        let middle = (left + right) / 2;

        let mut current = 1;
        let mut len = 0;

        for i in 0..n {
            len += l[i];
            if len > middle {
                current += 1;
                len = l[i];
            }
        }

        if current > m {
            left = middle;
        } else {
            right = middle;
        }
    }

    println!("{}", right - 1);
}
