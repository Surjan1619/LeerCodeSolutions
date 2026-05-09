use std::collections::HashMap;

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
        while idx <= s.len() - max_size {
            let temp = &s[idx..idx+max_size].chars().collect::<Vec<char>>();
            let mut substrings: Vec<String> = temp
                .chunks(size)
                .map(|chunk| chunk.iter().collect())
                .collect();
            substrings.sort();
            if substrings == words_sorted {res.push(idx as i32)}
            idx += 1;
        }
        res
            }
        }



fn main() {
    let ve : Vec<String>= ["fa","ba"].iter().map(|s| s.to_string()).collect();
    let a = Solution::find_substring("fafabafhefuesbafafaba".to_string(),ve);
    println!("{:?}", a);
}