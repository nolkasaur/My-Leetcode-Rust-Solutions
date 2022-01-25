// https://leetcode.com/problems/water-bottles/
// 1 ms, 2 MB

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut x = num_bottles;
        let mut res = num_bottles;
        let mut rem = 0;
        while x > 1 {
            if x / num_exchange == 0 { break; }
            let y = x % num_exchange;
            x /= num_exchange;
            res += x;
            x += y;
        }
        return res;
    }
}
