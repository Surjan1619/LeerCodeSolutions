use std::cmp;
struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut curr_sum = nums[0];
        for i in 0.. nums.len() {
            curr_sum = cmp::max(nums[i], nums[i] + curr_sum);
            if curr_sum > max_sum {max_sum = curr_sum}
        }
        max_sum

    }
}


fn main() {
   let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let a = Solution::max_sub_array(nums);
    println!("{}", a);
}
