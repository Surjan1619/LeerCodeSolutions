struct Solution;
impl Solution {
    
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 1 { let mut res = vec![];
            for i in 1..n {
                res.push(vec![i as i32])
            }
        return res;
        }


        vec![vec![]]
    }
}

fn main() {
    println!("Hello, world!");
}
