use itertools::iproduct;
use permutohedron::heap_recursive;
use proconio::input;
fn main() {
    input!{
        h: usize,
        w: usize,
        a: [[usize;w];h],
        b: [[usize;w];h]
    };
    let result = calculate_operations(&a, &b);

    println!("{}", result);
}
fn calculate_operations(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> i32 {
    let h = a.len();
    let w = a[0].len();
    
    // 行の順列を計算
    let mut rows: Vec<usize> = (0..h).collect();
    let mut min_operations = std::i32::MAX;

    heap_recursive(&mut rows, |row_permutation| {
        let mut new_a: Vec<Vec<usize>> = row_permutation.iter().map(|&i| a[i].clone()).collect();
                let mut cols: Vec<usize> = (0..w).collect();
        heap_recursive(&mut cols, |col_permutation| {
            let transformed_a: Vec<Vec<usize>> = col_permutation.iter().map(|&j| {
                new_a.iter().map(|row| row[j]).collect()
            }).collect();
            if transformed_a == *b {
                let row_operations = row_permutation.iter().enumerate().filter(|&(a, &b)| a != b).count() as i32;
                let col_operations = col_permutation.iter().enumerate().filter(|&(a, &b)| a != b).count() as i32;
                let total_operations = row_operations + col_operations;
                min_operations = min_operations.min(total_operations);
            }
        });
    });

    if min_operations == std::i32::MAX {
        -1
    } else {
        min_operations
    }   
}


