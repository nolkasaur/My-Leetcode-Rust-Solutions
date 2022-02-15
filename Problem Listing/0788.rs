// https://leetcode.com/problems/rotated-digits/
// 14 ms, 1.9 MB

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut res = 0;
        for num in 1..n+1 {
            let originalNum = num.to_string();
            let mut rotatedNum = String::from("");
            for digit in num.to_string().chars() {
                match digit {
                    '2' => rotatedNum.push('5'),
                    '5' => rotatedNum.push('2'),
                    '6' => rotatedNum.push('9'),
                    '9' => rotatedNum.push('6'),
                    '0' => rotatedNum.push('0'),
                    '1' => rotatedNum.push('1'),
                    '8' => rotatedNum.push('8'),
                    _ => break,
                }
            }
            if rotatedNum.len() == originalNum.len() && rotatedNum != originalNum {
                res += 1;
            }
        }
        return res;
    }
}
