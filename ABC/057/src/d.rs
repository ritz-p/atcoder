use proconio::input;
use std::collections::BTreeMap;
// TODO: solve
fn main(){
    input!{
        n: usize,
        a: usize,
        b: usize,
        v: [usize;n]
    };
    let mut bmap = BTreeMap::new();
    for value in v{
        *bmap.entry(value).or_insert(0) += 1;
    }
    let mut c = 0;
    let mut res = 0.0;
    println!("{:?}",bmap);
    for i in a..=b{
        let mut count = 0;
        let mut sum = 0.0;
        for (key,value) in bmap.iter().rev(){
            count += value;
            println!("{} {}",key,count);
            if count == i{
                sum += (key * value) as f64;
                if res < sum / i as f64{
                    res = sum / i as f64;
                    c = 1;
                }
                break;
            }else if count > i{
                sum += (key * (count - i)) as f64;
                if res == sum / i as f64{
                    c = c.max(count - i);
                }else if res < sum / i as f64{
                    res = sum / i as f64;
                    c = count - i;
                }
                break;
            }else if count < i{
                sum += (key * value) as f64;
            }
        }
    }
    println!("{}",res);
    println!("{}",c);
}
