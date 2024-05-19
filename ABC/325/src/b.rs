use proconio::input;

fn main(){
    input!{
        n: usize,
        wx: [(usize,usize);n]
    };
    let mut a = vec![0;24];
    for i in 0..24{
        for (w,x) in &wx{
            if (x + i) % 24 >= 9 && (x + i) % 24 <= 17{
                a[i] += w;
            }
        }
    }
    a.sort();

    println!("{}",a[23]);
}
