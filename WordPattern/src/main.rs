use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern : Vec<char> = pattern.chars().collect();
        let s : Vec<&str>= s.split(' ').collect::<Vec<&str>>();
        println!("patterns-{:?} s-{:?}", pattern, s);
        if pattern.len() != s.len() {return false}
        let mut maps = HashMap::<char, String>::new();
        for i in 0..pattern.len() {
            if let Some((key, val)) = maps.get_key_value(&pattern[i]) {
                if val == s[i] && key.to_string() == pattern[i].to_string(){continue}else { return false }
            }else{if maps.values().any(|v| v == s[i]){return false}else{
                maps.insert(pattern[i], s[i].to_string())};}
        }
        println!("map {:?}", maps);
        true
    }
}
fn main() {
    let s = Solution::word_pattern("aaaa".to_string(), "dog dog dog dog".to_string());
    // pattern = "abba", s = "dog cat cat dog"
    println!("{}", s);
}
