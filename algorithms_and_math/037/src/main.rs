use proconio::input;

fn main(){
    input!{
        points: [(f64,f64);4],
    };
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