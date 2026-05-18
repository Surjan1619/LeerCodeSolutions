use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 1 {return vec![vec![strs[0].to_string()]]; }
        
        vec![vec![]]
    }
}

fn main() {
    println!("Hello, world!");
}
