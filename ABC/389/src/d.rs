use proconio::input;

fn main() {
    input! {
        r: f64,
    };

    let res = circles(r);

    println!("{}", res);
}

fn circles(radius: f64) -> usize {
    let mut count = 0;
    let r2 = radius * radius;
    let mut max = radius as i32;

    for i in 0..(radius as i32) {
        while max >= 1 {
            let corners = [
                (i as f64, max as f64),
                (i as f64 + 1.0, max as f64),
                (i as f64, max as f64 + 1.0),
                (i as f64 + 1.0, max as f64 + 1.0),
            ];
            if corners.iter().all(|&(x, y)| {
                let dx = x - 0.5;
                let dy = y - 0.5;
                dx * dx + dy * dy <= r2
            }) {
                break;
            }
            max -= 1;
        }

        if max > 0 {
            count += max as usize;
        }
    }
    4 * count + 1
}
