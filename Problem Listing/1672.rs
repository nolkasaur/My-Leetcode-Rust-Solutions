// https://leetcode.com/problems/richest-customer-wealth/
// 3 ms, 2.3 MB

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut maxWealth: i32 = 0;
        let it = accounts.iter();
        for val in it {
            let sum: i32 = val.iter().sum();
            if sum > maxWealth {
                maxWealth = sum;
            }
        }
        return maxWealth;
    }
}
