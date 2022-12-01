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
    let bcx = b1-c1;
    let bcy = b2-c2;
    let r2 = (bcx*bcx) + (bcy*bcy);
    let tt = bcy*(b1-c1)+bcx*(b2-c2);
    let mut distance = 0.0;
    if tt < 0.0{
        distance = (c1-a1)*(c1-a1)+(c2-a2)*(c1-a2);
    }else if tt > r2{
        distance = (b1-a1)*(b1-a1)+(b2-a2)*(b1-a2);
    }else{
        distance = (bcx * (c1-a1) - bcy*(c2-a2))/r2;
    }

    println!("{}",distance.sqrt());
    
}