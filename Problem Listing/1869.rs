// https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/submissions/
// 2 ms, 2.2 MB

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut is1 = false;
        let mut count0 = 0;
        let mut count1 = 0;
        let mut max0 = 0;
        let mut max1 = 0;
        for x in s.chars() {
            if x == '1' {
                count1 += 1;
                if count0 > max0 { max0 = count0; }
                count0 = 0;
            } else {
                count0 += 1;
                if count1 > max1 { max1 = count1; }
                count1 = 0;
            }
        }
        if count1 > max1 { max1 = count1; }
        if count0 > max0 { max0 = count0; }
        return max1 > max0;
    }
}
