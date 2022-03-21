// https://leetcode.com/problems/maximum-number-of-coins-you-can-get/
// 28 ms, 3.3 MB

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort();
        let mut sum = 0;
        let max = piles.len();
        let mut x = 0;
        while x < max-max/3 {
            sum += piles[max-2-x];
            x+=2;
        }
        return sum;
    }
}
