struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx = nums1.len() - 1;
        let mut i = nums2.len() - 1;
        while idx >= nums1.len() - nums2.len(){
            nums1[idx] = nums2[i];
            if i == 0 {break}
            idx -= 1;
            i -= 1;
        }
        nums1.sort();
        println!("{:?}", nums1);
    }
}


fn main() {
    let mut nums1 = vec![1, 2, 3, 4, 5, 0, 0];
    let mut nums2 = vec![5, 4,];
    let a = Solution::merge(&mut nums1, 1 , &mut nums2, 2);
}
