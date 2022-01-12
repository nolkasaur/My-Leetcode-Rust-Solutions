// https://leetcode.com/problems/maximum-69-number/
// 0 ms, 2 MB

impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut c = num;
        while c > 9 {
            digits.push(c % 10);
            c /= 10;
        }
        digits.push(c);
        digits.reverse();
        for x in 0..digits.len() {
            if digits[x] == 6 { digits[x] = 9; break; }
        }
        
        let mut res = 0;
        for elem in digits {
            res *= 10;
            res += elem;
        }
        return res;
    }
}
