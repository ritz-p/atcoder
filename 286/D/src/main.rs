use proconio::input;

fn main(){
    input!{
        n: usize,
        x: usize,
        ab: [(usize,usize);n],
    };
    let mut max = 0;
    let mut flag = vec![false;x+1];
    for i in 0..n{
        max += ab[i].0*ab[i].1;
        // println!("{}",max);
        if max > x{
            max = x;
        }
        flag[ab[i].0] = true;
        for j in 1..=ab[i].1{
            for k in 0..max{
                if k+j*ab[i].0 > max{
                    break;
                }
                if flag[k]{
                    flag[k+j*ab[i].0] = true;
                }
                if flag[x]{
                    println!("Yes");
                    // println!("{:?}",flag);
                    return
                }
            }
        }
    }

    println!("No");
    // println!("{:?}",flag);
}