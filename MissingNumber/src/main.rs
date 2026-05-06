struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let ideal_arr = {nums.len() * (nums.len() + 1) / 2};
        let summ = nums.iter().sum::<i32>();
        ideal_arr as i32 - summ


    }
}

fn main() {
    let a = Solution::missing_number(vec![0, 1]);
    println!("{:?}", a);
}
