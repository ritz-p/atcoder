use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize;n],
        mut b: [usize;n]
    };
    a.sort();
    b.sort();
    let mut ac = 0;
    let mut count = n;
    for (index,e) in a.iter().rev().enumerate(){
        ac += e;
        if ac > x{
            count = count.min(index+1);
            break;
        }
    }
    let mut bc = 0;
    for (index,e) in b.iter().rev().enumerate(){
        bc += e;
        if bc > y{
            count = count.min(index+1);
            break;
        }
    }
    println!("{}",count);
}
