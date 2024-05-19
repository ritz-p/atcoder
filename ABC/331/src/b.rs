use proconio::input;

fn main(){
    input!{
        n: isize,
        s: isize,
        m: isize,
        l: isize,
    };
    let mut min_cost = isize::MAX;
    for i in 0..=n/6+1 {
        for j in 0..=n/8+1 {
            for k in 0..=n/12+1{
                if i * 6 + j * 8 + k * 12 >= n{
                    let cost = i as isize * s + j as isize * m + k as isize * l;
                    min_cost = min_cost.min(cost);
                    // println!("{} {} {} {}",i,j,k,min_cost);
                }
            }
        }
    }

    println!("{}",min_cost);
}
