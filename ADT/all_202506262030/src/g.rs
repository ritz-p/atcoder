use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut l: [usize;n]
    };
    l = l.iter().map(|e| e + 1).collect();
    let mut left = l.iter().max().unwrap() - 1;
    let mut right = l.iter().sum::<usize>();

    while left + 1 < right {
        let mid = (left + right) / 2;

        let mut wid = 1;
        let mut len = 0;

        for i in 0..n {
            len += l[i];
            if len > mid {
                wid += 1;
                len = l[i];
            }
        }

        if wid > m {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", right - 1);
}
