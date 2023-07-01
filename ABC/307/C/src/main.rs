use proconio::{input, marker::Chars};

fn main() {
    input! {
        ha: usize,
        _: usize,
        grid_a: [Chars; ha],
        hb: usize,
        _: usize,
        grid_b: [Chars; hb],
        hx: usize,
        _: usize,
        grid_x: [Chars; hx],
    }
    // grid_xは10*10に収まる
    // 黒いマスは全て利用されるはず

    let trimed_a = trim(&grid_a);
    let trimed_b = trim(&grid_b);
    let trimed_x = trim(&grid_x);
    // xの枠の中でaとbを全探索
    // a,bともにxの枠の中に収まらない場合はNG
    if trimed_a.len() > trimed_x.len()
        || trimed_a[0].len() > trimed_x[0].len()
        || trimed_b.len() > trimed_x.len()
        || trimed_b[0].len() > trimed_x[0].len()
    {
        println!("No");
        return;
    }
    let mut res = false;
    for i in 0..=(trimed_x.len() - trimed_a.len()) {
        for j in 0..=(trimed_x[0].len() - trimed_a[0].len()) {
            for k in 0..=(trimed_x.len() - trimed_b.len()) {
                for l in 0..=(trimed_x[0].len() - trimed_b[0].len()) {
                    let mut target = vec![vec!['.'; trimed_x[0].len()]; trimed_x.len()];
                    for m in 0..trimed_a.len() {
                        for n in 0..trimed_a[0].len() {
                            if trimed_a[m][n] == '#' {
                                target[i + m][j + n] = '#';
                            }
                        }
                    }
                    for m in 0..trimed_b.len() {
                        for n in 0..trimed_b[0].len() {
                            if trimed_b[m][n] == '#' {
                                target[k + m][l + n] = '#';
                            }
                        }
                    }

                    if target == trimed_x {
                        res = true;
                    }
                }
            }
        }
    }

    println!("{}", if res { "Yes" } else { "No" });
}

// 黒いマスのある範囲のGridを取り出す
fn trim(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut top = std::usize::MAX;
    let mut bottom = 0;
    let mut left = std::usize::MAX;
    let mut right = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                top = top.min(i);
                bottom = bottom.max(i);
                left = left.min(j);
                right = right.max(j);
            }
        }
    }
    let mut res = vec![vec!['.'; right - left + 1]; bottom - top + 1];
    for i in top..=bottom {
        for j in left..=right {
            res[i - top][j - left] = grid[i][j];
        }
    }
    res
}
