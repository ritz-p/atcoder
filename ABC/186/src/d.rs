use proconio::input;
fn main(){
    input!{
        n: usize,
        mut a: [isize;n]
    };
    let mut res = 0;
    a.sort();
    let mut p = 0;
    let mut m = 0;
    let mut z = 0;
    for v in &a{
        if v > &0{
            p += 1;
        }else if v == &0{
            z += 0;
        }else{
            m += 1;
        }
    }
    for (index,v) in a.iter().rev().enumerate(){
        if p == 0{
            break;
        }
        res += v * (p - index - 1 + m + z) as isize;
        res -= v * index as isize;
        if index == p-1{
            break;
        }
    }
    for (index,v) in a.iter().enumerate(){
        if m == 0{
            break;
        }
        res += v * (m - index - 1 + p + z) as isize * -1;
        res -= v * index as isize * -1;
        if index == m-1{
            break;
        }
    }
    println!("{}",res);
}
