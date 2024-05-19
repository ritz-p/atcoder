use proconio::input;

fn main(){
    input!{
        n: usize
    };
    let mut satisfied = vec![vec![];n];
    for _ in 0..n{
        input!{
            a: usize,
            b: usize,
        }
        satisfied[a-1].push(b);
    }
    for i in 0..n{
        if satisfied[i].len() >= 2{
            satisfied[i].sort_by(|a, b| b.cmp(a));
        }
    }
    let mut maxes = vec![];

    for i in 0..n{
        if satisfied[i].len() == 1{
            maxes.push(satisfied[i][0]);
        }else if satisfied[i].len() >= 2{
            maxes.push(satisfied[i][0]);
            maxes.push(satisfied[i][1]/2);
        }
    }
    maxes.sort_by(|a, b| b.cmp(a));
    println!("{}",maxes[0]+maxes[1]);
}
