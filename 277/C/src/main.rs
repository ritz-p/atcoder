use proconio::input;
fn main(){
    input!{
        n: usize,
        mut tuple: [(usize,usize);n]
    };

    // for i in 0..n{
    //     if tuple[i].0 > tuple[i].1{
    //         let tmp1 = tuple[i].0;
    //         tuple[i].0 = tuple[i].1;
    //         tuple[i].1 = tmp1;
    //     }
    //     for j in i+1..n{
    //         if tuple[i].0 > tuple[j].0{
    //             let tmp2 = tuple[i];
    //             tuple[i] = tuple[j];
    //             tuple[j] = tmp2;
    //         }
    //         println!("{},{}",tuple[j].0,tuple[j].1);
    //     }
    // }
    let mut stage = vec![false;1000000001];
    stage[0] = true;
    stage[1] = true;
    for i in 0..n{
        if stage[tuple[i].0]{
            stage[tuple[i].1] = true;
            // println!("{} {}",tuple[i].0,tuple[i].1);
        }
    }
    let mut res = 1;
    for i in 0..n{
        if stage[tuple[i].1]{
            if res < tuple[i].1{
                res = tuple[i].1;
            }
        }
    }
    println!("{}",res);

}