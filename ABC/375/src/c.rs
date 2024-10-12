use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut arr: [Chars;n]
    };
    let mut res = vec![vec![' '; n]; n];
    for x in 0..n {
        for y in 0..n {
            let k = (x + 1).min(y + 1).min(n - x).min(n - y);
            let mut nx = x;
            let mut ny = y;
            for _i in 0..k % 4 {
                let tx = ny;
                let ty = n - 1 - nx;
                nx = tx;
                ny = ty;
            }
            res[nx][ny] = arr[x][y];
        }
    }

    for row in res {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}
