impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;
        while left < right {
            let temp = height[left].min(height[right]) * (right - left) as i32;
            if temp > result { result = temp; }else{
                println!("left:{},   right:{} ", height[left], height[right]);
                if height[left] < height[right] {
                    left = left + 1;
                }else{
                    right = right - 1;
                }
            }
        }
        result
    }
}