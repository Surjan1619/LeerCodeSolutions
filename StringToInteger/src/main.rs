struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        println!("{}", i64::MAX);
        let s = s.chars().collect::<Vec<char>>();
        let mut state = 2;
        let mut result = 0;
        let mut started = false;
        for  num in s {
            if result >= i32::MAX as i64 {break}
            match num.to_digit(10) {
                Some(x) => {result = result * 10 + x as i64;
                started = true;},
                None => {if num.to_string() == " " && !started{continue;}
                    else if num.to_string() ==  "-" && state == 2 && !started{ state = -1; started = true}
                    else if num.to_string() == "+" && state == 2 && !started{state = 1; started = true}
                    else {break} }
                }
            }
        if result > i32::MAX as i64 {
            if state != -1 { return i32::MAX} else {return i32::MIN }
        }else {if state != -1 {return result as i32} else{return result as i32 * -1}}
    }

}
fn main() {
    let a = Solution::my_atoi("9223372036854775808".to_string());
    println!("{}", a);
}