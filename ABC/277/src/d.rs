use proconio::input;

// mod 関係ほぼなし
fn main() {
    input!{
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut tmp = a[0];
    let mut v = Vec::new();
    // 2 以上差があると途切れる
    for i in 1..n {
        if a[i] > a[i-1] + 1 {
            v.push(tmp);
            tmp = a[i];
        }
        else {
            tmp += a[i];
        }
    }
    v.push(tmp);
    // 最後尾の数 n+1 = m かつ最初の数 k = 0 のとき 条件成立 ((x+1) mod m = 0 のため)
    if a[0] == 0 && a[a.len()-1] == m-1 && v.len() > 1 {
        v.push(v[0]+v[v.len()-1])
    }
    // println!("{}",a.iter().sum::<usize>());
    // println!("{:?}",a);
    // println!("{}",v.iter().max().unwrap());
    // println!("{:?}",v);
    println!("{}", a.iter().sum::<usize>() - v.iter().max().unwrap());
}