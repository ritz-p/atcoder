use proconio::input;

fn main(){
    input!{
        points: [(f64,f64);4],
    };
    if (y3 - y1) * (x1 - x2) == (y1 - y2) * (x3 - x1)
    && (y4 - y1) * (x1 - x2) == (y1 - y2) * (x4 - x1)
{
    let mut a = (x1, y1);
    let mut b = (x2, y2);
    let mut c = (x3, y3);
    let mut d = (x4, y4);

    if a > b {
        mem::swap(&mut a, &mut b);
    }

    if c > d {
        mem::swap(&mut c, &mut d);
    }

    if cmp::max(a, c) <= cmp::min(b, d) {
        println!("Yes");
    } else {
        println!("No");
    }

    return;
}
    let s1 = (points[0].0 - points[1].0) * (points[2].1 - points[0].1) + (points[0].1 - points[1].1) * (points[0].0 - points[2].0);
    let s2 = (points[0].0 - points[1].0) * (points[3].1 - points[0].1) + (points[0].1 - points[1].1) * (points[0].0 - points[3].0);
    let t1 = (points[2].0 - points[3].0) * (points[0].1 - points[2].1) + (points[2].1 - points[3].1) * (points[2].0 - points[0].0);
    let t2 = (points[2].0 - points[3].0) * (points[1].1 - points[2].1) + (points[2].1 - points[3].1) * (points[2].0 - points[1].0);
    // println!("{},{},{},{}",s1,s2,t1,t2);
    if s1 * s2 < 0.0 && t1 * t2 < 0.0{
        println!("Yes");
    }else{
        println!("No");
    }

}