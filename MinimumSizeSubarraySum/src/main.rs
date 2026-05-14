struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() < target { return 0}
        else if nums.len() > 1{
            let mut l = 0;
            let mut r = 1;
            let mut curr_sum = nums[l] + nums[r];
            let mut min_len = i32::max_value();
            let mut curr_len = 2;
            while l < nums.len() {
                println!("l: {}, r: {}, curr_sum: {}, cur len {curr_len}", l, r, curr_sum);
                if nums[r] >= target || nums[l] >= target{return 1}
                if curr_sum >= target {
                    if curr_len < min_len {min_len = curr_len;}
                    if r - l > 1 {
                        curr_sum -= nums[l];
                        l += 1;
                        curr_len -= 1;
                    }else{
                        if r < nums.len() - 1 {
                        r += 1;
                        curr_len += 1;
                        curr_sum += nums[r];}else{break}
                    }
                }else{
                    if r < nums.len() - 1 {
                        r += 1;
                        curr_sum += nums[r];
                        curr_len += 1;
                    }else{break}
                }
            }
            min_len
        }else{return 1}
    }
}
fn main() {
    let nums = vec![8];
    let a = Solution::min_sub_array_len(7,nums);
    println!("{}",a);
}
