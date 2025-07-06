use proconio::input;

fn main() {
    input! { t: usize }
    let mut res = Vec::with_capacity(t);

    for _ in 0..t {
        input! { n: usize, a: [isize; n] }
        if n <= 2 {
            res.push("Yes");
            continue;
        }
        let pos = a.iter().filter(|&&v| v == a[0]).count();
        let neg = a.iter().filter(|&&v| v == -a[0]).count();
        if pos + neg == n && pos.min(neg) == n / 2 {
            res.push("Yes");
            continue;
        }

        let mut abs = a
            .iter()
            .map(|&v| (v, v.abs()))
            .collect::<Vec<(isize, isize)>>();
        abs.sort_by(|k, v| k.1.cmp(&v.1));

        if abs.windows(3).any(|w| w[1].0 * w[1].0 != w[0].0 * w[2].0) {
            res.push("No");
            continue;
        }
        res.push("Yes");
    }

    println!("{}", res.join("\n"));
}
