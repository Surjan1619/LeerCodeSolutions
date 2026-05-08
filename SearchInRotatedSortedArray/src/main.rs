struct Solution {
}
impl Solution {
    pub fn cut(arr : Vec<i32>,target: i32, left: usize, right: usize) -> i32 {
        let mid = (left + right) / 2;
        if arr[left] == target{return left as i32}
        if arr[right] == target {return right as i32}
        if arr[mid] == target {return mid as i32}
        if arr[mid..right].contains(&target){
             return Self::cut(arr,target,mid, right);
        }else{
            return Solution::cut(arr,target,left, mid);
        }
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.contains(&target){
        let l :usize = 0;
        let r = nums.len() - 1;
        let res = Self::cut(nums,target,l,r);
        return res;}
        return -1;
1
    }
}
fn main() {
    let nums = vec![1];
    let a = Solution::search(nums, 4);
    println!("{}",a);
}