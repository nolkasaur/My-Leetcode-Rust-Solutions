// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut prod: u32 = 1;
        let mut add: u32 = 0;
        let str: String = n.to_string();
        for x in 0..str.len() {
            prod *= str.chars().nth(x).unwrap().to_digit(10).unwrap();
            add += str.chars().nth(x).unwrap().to_digit(10).unwrap();
        }
        return (prod - add) as i32;
    }
}
