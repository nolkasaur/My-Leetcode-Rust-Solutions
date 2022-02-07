// https://leetcode.com/problems/three-divisors/
// 4 ms, 2 MB

impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut c = n;
        let mut count = 0;
        while c > 0 {
            if n % c == 0 {
                count += 1;
                if count == 4 {
                    return false;
                }
            }
            c -= 1;
        }
        if count < 3 {
            return false
        } else {
            return true
        }
    }
}
