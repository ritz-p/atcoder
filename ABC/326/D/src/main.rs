use proconio::input;
use std::collections::HashSet;

fn main() {
    input!{
        n: usize,
        r: String,
        c: String
    };
    let mut perm = Vec::new();
    let mut used = vec![false; n];
    let mut result = Vec::new();
    
    generate_permutations(&mut perm, &mut used, &mut result, &r, &c);
    
    if result.is_empty() {
        println!("No solution");
    } else {
        println!("{}", result[0].iter().collect::<String>());
    }
}

fn is_valid_permutation(perm: &Vec<char>, r: &String, c: &String) -> bool {
    let mut row_seen = HashSet::new();
    let mut col_seen = HashSet::new();
    
    for i in 0..perm.len() {
        if row_seen.contains(&perm[i]) || col_seen.contains(&perm[i]) {
            return false;
        }
        
        let r_idx = match r.chars().position(|x| x == perm[i]){
            None => 0,
            _ => r.chars().position(|x| x == perm[i]).unwrap()
        };
        let c_idx = match c.chars().position(|x| x == perm[i]){
            None => 0,
            _ => c.chars().position(|x| x == perm[i]).unwrap()
        };
        
        if r_idx > i || c_idx > i {
            return false;
        }
        
        row_seen.insert(perm[i]);
        col_seen.insert(perm[i]);
    }
    
    true
}

fn generate_permutations(perm: &mut Vec<char>, used: &mut Vec<bool>, result: &mut Vec<Vec<char>>, r: &String, c: &String) {
    if perm.len() == r.len() {
        if is_valid_permutation(perm, r, c) {
            result.push(perm.clone());
        }
        return;
    }
    
    for i in 0..used.len() {
        if !used[i] {
            used[i] = true;
            perm.push((i as u8 + 'A' as u8) as char);
            generate_permutations(perm, used, result, r, c);
            perm.pop();
            used[i] = false;
        }
    }
}

