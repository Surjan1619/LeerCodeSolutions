struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let vnum1 = num1.chars().collect::<Vec<char>>();
        let vnum2 = num2.chars().collect::<Vec<char>>();
        let mut res = 0;
        let mut degree : i32 = 1;
        for i in (0..vnum1.len()).rev() {
            let i_n = vnum1[i] as i32 - '0' as i32;
            let mut rememb = 0;
            for j in (0..vnum2.len()).rev() {
                let j_n = vnum2[j] as i32 - '0' as i32;
                let mut multip = i_n * j_n;
                if rememb == 1 {multip += 1; rememb = 0}
                if multip >= 10 {rememb = 1; multip -= 10}
                res += multip * degree;
                degree = degree.pow(2);
                println!("{}", degree);
                println!("i-{} j-{} multip: {}, res-{}", i_n, j_n, multip, res);
            }//SXALA HAVAQAC PTI STRING SARQEI HANE I32 I MEJ EM AVELACRE POXI

        }
        "".to_string()
    }
}//[8, 6, 3, 1, 2, 1, 9, 6, 5, 4]

fn main() {
    let a = Solution::multiply("123".to_string(), "456".to_string());
}
