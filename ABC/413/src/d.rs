use proconio::input;

fn main() {
    input! { t: usize }
    let mut res = Vec::with_capacity(t);

    for _ in 0..t {
        input! { n: usize, mut a: [isize; n] }
        if n <= 2 {
            res.push("Yes");
            continue;
        }
        a.sort_unstable();
        let mut abs = a.iter().map(|&v| v.abs()).collect::<Vec<isize>>();
        abs.sort_unstable();

        if abs.windows(2).any(|w| w[0] == w[1]) {
            res.push("No");
            continue;
        }
        if abs.windows(3).any(|w| w[1] * w[1] != w[0] * w[2]) {
            res.push("No");
            continue;
        }
        if abs[0] == abs[n - 1] {
            let pos = a.iter().filter(|&&v| v > 0).count();
            let neg = n - pos;
            if pos == n || neg == n {
                res.push("Yes");
            } else {
                let pos1 = (n + 1) / 2;
                let pos2 = n / 2;
                if (pos == pos1 && neg == pos2) || (pos == pos2 && neg == pos1) {
                    res.push("Yes");
                } else {
                    res.push("No");
                }
            }
        } else {
            let pos = a.iter().filter(|&&v| v > 0).count();
            let neg = n - pos;
            if pos == n || neg == n {
                res.push("Yes");
            } else {
                res.push("No");
            }
        }
    }

    println!("{}", res.join("\n"));
}
