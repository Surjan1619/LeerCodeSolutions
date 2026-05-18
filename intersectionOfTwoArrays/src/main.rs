use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set: HashSet<i32> = HashSet::from_iter(nums1);
        let mut res = HashSet::new();
        for i in 0..nums2.len() {
            if set.contains(&nums2[i]) {res.insert(nums2[i]);}
        }
        res.into_iter().collect()
    }
}


fn main() {
    let a = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
    println!("{:?}", a);
}
