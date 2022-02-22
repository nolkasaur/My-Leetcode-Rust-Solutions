// https://leetcode.com/problems/count-integers-with-even-digit-sum/
// 0 ms, 2 MB

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut res = 0;
        for x in 1..num+1 {
            if sumDigits(x) % 2 == 0 {
                res += 1;
            }
        }
        return res;
    }
}

pub fn sumDigits(n: i32) -> i32 {
    if n >= 10 {
        return n%10 + sumDigits(n/10)
    } else {
        return n;
    }
}
