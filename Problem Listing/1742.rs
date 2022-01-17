// https://leetcode.com/problems/maximum-number-of-balls-in-a-box/
// 20 ms, 2 MB

use std::collections::HashMap;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut nrBalls: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        for x in low_limit..high_limit+1 {
            let count = nrBalls.entry(sumDigits(x)).or_insert(0);
            *count += 1;
            if *count > max { max = *count; }
        }
        return max;
    }
}

pub fn sumDigits(num: i32) -> i32 {
    if num >= 10 {
        return num%10 + sumDigits(num/10)
    } else {
        return num%10;
    }
}
