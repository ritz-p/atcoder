use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [usize;n],
        b: [usize;m]
    };
    let mut c = vec![];
    for e in &a{
        c.push((e,'a'));
    }
    for e in &b{
        c.push((e,'b'));
    }
    c.sort_by(|a,b|a.0.cmp(b.0));

    for i in 0..c.len()-1{
        if c[i].1 == 'a' && c[i+1].1 == 'a'{
            println!("Yes");
            return;
        }
    }
    println!("No");
}
