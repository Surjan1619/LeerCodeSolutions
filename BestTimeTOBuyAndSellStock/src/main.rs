struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut idx = 1;
        let mut curr_profit = 0;
        let mut buy = prices[0];
        let mut max_profit = 0;
        while idx < prices.len() {
            if prices[idx] > buy {
                curr_profit = prices[idx] - buy;
            }else {
                buy = prices[idx];
            }
            if curr_profit > max_profit {max_profit = curr_profit;}
        idx += 1;
        }
    max_profit
    }

}

fn main() {
    let a = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("{}", a);
    let mut i = 0;


