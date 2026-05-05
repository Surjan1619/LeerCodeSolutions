impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.contains(&val) {
            let mut index = 0;
            while index < nums.len(){
                if nums[index] == val {
                    nums.swap_remove(index);
                    println!("{index}");
                    println!("{:?}", nums);

                }else{
                    index += 1;
                }
            };
            return nums.len() as i32;
            println!("{:?}", nums);
        }else { return nums.len() as i32; }
        }
    }
