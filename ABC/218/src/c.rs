use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        mut s: [Chars;n],
        t: [Chars;n]
    };

    let mut s_normalized = normalize(&s, n);
    let t_normalized = normalize(&t, n);
    for _ in 0..4 {
        if is_equal(&s_normalized, &t_normalized, n) {
            println!("Yes");
            return;
        }
        s_normalized = normalize(&rotate(&s_normalized, n), n);
    }

    println!("No");
}

fn rotate(grid: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut rotated = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            rotated[j][n - i - 1] = grid[i][j];
        }
    }
    rotated
}

fn normalize(grid: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut min_row = n;
    let mut min_col = n;

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == '#' {
                if i < min_row {
                    min_row = i;
                }
                if j < min_col {
                    min_col = j;
                }
            }
        }
    }

    let mut normalized = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == '#' {
                normalized[i - min_row][j - min_col] = '#';
            }
        }
    }
    normalized
}

fn is_equal(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>, n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    true
}
