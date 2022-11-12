use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
 
fn main() {
    input! {
        n: usize,
        hash: [(usize, usize); n]
    }
 
    let mut ladder = HashMap::new();
    let mut check_sheet = HashSet::new();
 
    for (a, b) in hash.iter() {
        if !ladder.contains_key(a) {
            ladder.insert(*a, Vec::new());
        }
        ladder.get_mut(a).unwrap().push(*b);
        if !ladder.contains_key(b) {
            ladder.insert(*b, Vec::new());
        }
        ladder.get_mut(b).unwrap().push(*a);
    }
    // println!("{:?}",ladder);
    if !ladder.contains_key(&1) {
        println!("1");
        return;
    }
 
    search(&ladder, &mut check_sheet, &1);
    println!("{}", check_sheet.iter().max().unwrap());
}

// 再帰苦手なのでよく読む
// 1 を入れているため 1 とつながっているところしか再帰しない
// 以下のような入力だと 11 6 は再帰に入らない
// 5
// 1 3
// 3 5
// 4 2
// 5 10
// 11 6
fn search(ladder: &HashMap<usize, Vec<usize>>, check_sheet: &mut HashSet<usize>, this_floor: &usize) {
    if check_sheet.contains(this_floor) {
        return;
    }
    check_sheet.insert(*this_floor);
 
    for i in ladder.get(this_floor).unwrap() {
        // println!("{},{:?}",*this_floor,ladder.get(this_floor));
        search(ladder, check_sheet, i);
    }
}