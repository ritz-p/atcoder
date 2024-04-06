use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let mut point:Vec<(usize,isize,isize)> = vec![];
    for i in 1..=n{
        input!{
            x: isize,
            y: isize,
        }
        point.push((i,x,y));
    }
    for i in 0..n{
        let mut cur = i;
        let mut m = 0;
        for j in 0..n{
            if i == j{
                continue;
            }
            let cul_m = (point[i].1 - point[j].1) * (point[i].1 - point[j].1) + (point[i].2 - point[j].2) * (point[i].2 - point[j].2);
            if cul_m > m{
                m = cul_m;
                cur = point[j].0;
            }
        }
        println!("{}",cur);
    }

}
