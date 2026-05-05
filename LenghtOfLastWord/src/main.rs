impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
    let vec = s.as_str().split_whitespace().collect::<Vec<&str>>();
        vec[vec.len() - 1].len() as i32


    }
}