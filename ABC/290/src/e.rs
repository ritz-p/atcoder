use proconio::input;

fn main(){
    input!{
        n: usize,
        row: [usize;n],
    };
    let mut part_sequence = vec![vec![];n+1];
    for i in 1..=n{
        part_sequence[row[i-1]].push(i);
    }
    let mut res = 0;
    for i in 1..=n{
        res += (n + 1 - i) * (i / 2);
    }
    for i in 1..=n{
        let mut l = 0 as isize;
        let mut r = part_sequence[i].len() as isize - 1;
        while l < r{
            if part_sequence[i][l as usize] < n + 1 - part_sequence[i][r as usize]{
                res -= (r - l) as usize * part_sequence[i][l as usize];
                l += 1;
            }else{
                res -= (r - l) as usize * (n + 1 - part_sequence[i][r as usize]);
                r -= 1;
            }
        }
    }
    println!("{}",res);
}