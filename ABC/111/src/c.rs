use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize;n]
    };
    let mut odd = vec![(0, 0); 100001];
    let mut even = vec![(0, 0); 100001];
    for (index, e) in v.iter().enumerate() {
        if index % 2 == 0 {
            odd[*e].0 += 1;
        } else {
            even[*e].0 += 1;
        }
    }
    for i in 0..100001 {
        odd[i].1 = i;
        even[i].1 = i;
    }
    odd.sort_by(|a, b| b.0.cmp(&a.0));
    even.sort_by(|a, b| b.0.cmp(&a.0));
    let odd_sum = odd.iter().map(|(x, _)| x).sum::<usize>();
    let even_sum = even.iter().map(|(x, _)| x).sum::<usize>();
    let mut res = 0;

    if odd[0].1 == even[0].1 {
        if odd[0].0 > even[0].0 {
            res += odd_sum - odd[0].0;
            res += even_sum - even[1].0;
        } else if odd[0].0 < even[0].0 {
            res += odd_sum - odd[1].0;
            res += even_sum - even[0].0;
        } else {
            if odd[1].0 > even[1].0 {
                res += odd_sum - odd[1].0;
                res += even_sum - even[0].0;
            } else {
                res += odd_sum - odd[0].0;
                res += even_sum - even[1].0;
            }
        }
    } else {
        res += odd_sum - odd[0].0;
        res += even_sum - even[0].0;
    }
    println!("{}", res);
}
