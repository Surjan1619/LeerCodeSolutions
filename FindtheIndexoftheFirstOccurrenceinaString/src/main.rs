impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.contains(&needle) {
         if haystack.len() == needle.len() {
             return 0
         }
            let arr: Vec<char> = haystack.chars().collect();
            for i in 0..arr.len(){
                let current_slice : String = arr[i..i + needle.len()].into_iter().collect();
                if current_slice == needle {
                    return i as i32;
                }}
            -1
        }else{
        -1
        }
    }
}