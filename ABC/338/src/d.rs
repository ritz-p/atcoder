use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        mut x: [usize;m]
    };
    for i in &mut x{
        *i -= 1;
    }
    let mut diffs:Vec<isize> = vec![0;m-1];
    for i in 0..m-1{
        diffs[i] = (x[i] as isize - x[i+1] as isize).abs();
    }
    let mut ans:isize = 0;
    for i in 0..m-1{
        ans += diffs[i];
    }

    let mut idxs:Vec<Vec<isize>> = vec![vec![];n];

    for i in 0..m{
        idxs[x[i]].push(i as isize);
    }

    let mut current = ans;

    for i in 0..n{
        for j in 0..idxs[i].len(){
            let val = idxs[i][j];

            if val > 0{
                let prev = diffs[val as usize - 1];
                current = current - prev + (n as isize - prev);
                diffs[val as usize - 1] = n as isize - prev;
            }
            if val < m as isize - 1{
                let prev = diffs[val as usize];
                current = current - prev + (n as isize - prev);
                diffs[val as usize] = n as isize - prev;
            }
        }
        ans = ans.min(current);
    }

    println!("{}",ans);
}
