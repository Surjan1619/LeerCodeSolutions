use std::collections::HashMap;
use std::mem::needs_drop;

struct Solution;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut words_sorted = words.clone();
        words_sorted.sort();
        println!("sorted words{:?}", words_sorted);
        let size = words[0].len();
        if s.len() < words.len() * size{return vec![]}
        let mut idx = 0;
        let max_size = words.len() * size;
        let mut res : Vec<i32> = Vec::new();
        let mut hashmap: HashMap<i32, String> = HashMap::new();
        while idx < max_size {
            
        }
        res
            }
        }



fn main() {
    let ve : Vec<String>= ["fa","ba"].iter().map(|s| s.to_string()).collect();
    let a = Solution::find_substring("fafabafhefuesbafafaba".to_string(),ve);
    println!("{:?}", a);
}