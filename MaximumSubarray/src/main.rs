use std::cmp;
struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 1;
        let mut sum = nums[0] + nums[1];
        while l < r{
            
        }
        sum

    }
}


fn main() {
   let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let a = Solution::max_sub_array(nums);
    println!("{}", a);
}
