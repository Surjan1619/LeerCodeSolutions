impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1 { return 1}
        let mut r = 1;
        let mut l = 0;
        let mut count = 1;
        let mut res = "".to_string();
        while l < chars.len() && r < chars.len() {
            if chars[l] == chars[r] {
                count += 1;
                r += 1
            }else{
                if count == 1 {res.push(chars[l])}else{
                res.push_str(format!("{}{}", chars[l], count).as_str());}
                l = r.clone();
                r += 1;
                count = 1;
        }
        }
        if count == 1 {res.push(chars[l])}else{
        res.push_str(format!("{}{}", chars[l], count).as_str());}
        *chars = res.chars().collect();
        println!("{}", res);
        res.len() as i32
        }
    }