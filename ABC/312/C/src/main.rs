use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        mut a: [usize;n],
        mut b: [usize;m]
    };

    let mut wa = 0;
    let mut ac = 1001001001;

    while wa+1<ac{
        let wj = (wa + ac)/2;
        let mut na = 0;
        let mut nb = 0;
        for i in 0..n{
            if a[i] <= wj{
                na += 1;
            }
        }
        for i in 0..m{
            if b[i] >= wj{
                nb += 1;
            }
        }
        if na >= nb{
            ac = wj;
        }else{
            wa = wj;
        }
    }
    println!("{}",ac);
}
