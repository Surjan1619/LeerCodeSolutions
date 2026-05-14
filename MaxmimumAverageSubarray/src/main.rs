struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max_sum : f64 = f64::MIN;
        if nums.len() == k as usize {
            return nums.iter().sum::<i32>() as f64 / k as f64;
        }else{
            let mut idx = 0;
            let mut curr_sum :f64;
            while idx <= nums.len() - k as usize{
                curr_sum = nums[idx..idx+k as usize].iter().sum::<i32>() as f64 / k as f64;
                if curr_sum > max_sum {
                    max_sum = curr_sum;
                }
                idx += 1;
            }
        }
    max_sum
    }
}


fn main() {
    let nums = vec![0,1,1,3,3];
    let a = Solution::find_max_average(nums, 4);
    println!("Max sum: {}", a);
}
