// https://leetcode.com/problems/maximum-ice-cream-bars/
// 72 ms, 3.8 MB

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort();
        let mut res = 0;
        let mut spent = 0;
        for x in costs {
            if spent + x <= coins {
                res +=1;
                spent += x;
            }
        }
        return res;
    }
}
