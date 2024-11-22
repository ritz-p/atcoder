use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize;n]
    };
    let mut map = HashMap::new();
    let mut l = 0;
    let mut res = 0;
    let mut i = 0;

    while i + 1 < n {
        if arr[i] == arr[i + 1] {
            let num = arr[i];
            *map.entry(num).or_insert(0) += 2;
            let mut j = i + 2;
            let mut valid = true;

            while j + 1 < n && arr[j] == arr[j + 1] {
                let next_num = arr[j];
                *map.entry(next_num).or_insert(0) += 2;
                while map[&next_num] > 2 {
                    let l_num = arr[l];
                    map.entry(l_num).and_modify(|e| *e -= 2);
                    if map[&l_num] == 0 {
                        map.remove(&l_num);
                    }
                    l += 2;
                }
                for k in (l..j).step_by(2) {
                    if arr[k] != arr[k + 1] {
                        valid = false;
                        l = k + 2;
                        break;
                    }
                }

                if valid && map.values().all(|&v| v == 2) {
                    res = res.max(j + 2 - l);
                }

                j += 2;
            }

            if map.values().all(|&v| v == 2) {
                res = res.max(j - l);
            }
            map.clear();
            l = i + 2;
            i += 2;
        } else {
            map.clear();
            i += 1;
            l = i;
        }
    }
    println!("{}", res);
}
