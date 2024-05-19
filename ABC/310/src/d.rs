use proconio::input;
use proconio::marker::Usize1;

fn main(){
    input!{
        n: usize,
        t: usize,
        m: usize,
        ab: [(Usize1,Usize1); m]
    };

    let mut p = vec![0;n+1];

    for (a,b) in ab{
        p[a] = p[a] | 1 << b;
        p[b] = p[b] | 1 << a;
    }

    let mut teams = vec![0;t];

    let ans = dfs(n,t,&p,&mut teams,0);
    println!("{}",ans);
}

pub fn dfs(n: usize, t: usize, p: &Vec<usize>, teams: &mut Vec<usize> , k:usize) -> usize{
    if k == n{
        return if teams.iter().all(|&x| x != 0){
            1
        }else{
            0
        };
    }
    let mut ret = 0;
    for i in 0..t{
        if (teams[i] & p[k]) == 0{
            let tmp = teams[i];
            teams[i] = teams[i] | 1 << k;
            ret += dfs(n,t,p,teams,k+1);
            teams[i] = tmp;
            if tmp == 0{
                break;
            }
        }
    }
    ret
}
