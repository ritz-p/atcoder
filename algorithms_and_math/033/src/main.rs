use proconio::input;

fn main(){
    input!{
        a1: f64,
        a2: f64,
        b1: f64,
        b2: f64,
        c1: f64,
        c2: f64,
    };
    let bcx = c1-b1;
    let bcy = c2-b2;
    let r2 = (bcx*bcx) + (bcy*bcy);
    let tt = (bcx*(b1-a1)+bcy*(b2-a2)) * -1.0;
    let mut distance = 0.0;
    if tt < 0.0{
        distance = (b1-a1)*(b1-a1)+(b2-a2)*(b2-a2);
    }else if tt > r2{
        distance = (c1-a1)*(c1-a1)+(c2-a2)*(c2-a2);
    }else{
        let f = (bcx*(b2-a2))-(bcy*(b1-a1));
        distance = (f*f)/r2;
    }
    // println!("r2={},tt={},distance={}",r2,tt,distance);
    // println!("{}",((b1-a1)*(b1-a1)+(b2-a2)*(b2-a2)).sqrt());
    // println!("{}",((c1-a1)*(c1-a1)+(c2-a2)*(c2-a2)).sqrt());
    println!("{}",distance.sqrt());
}