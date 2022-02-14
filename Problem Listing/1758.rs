// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/
// 3 ms, 2.2 MB

use std::cmp;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut res0 = 0;
        let mut res1 = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 && c != '0' { res0 += 1; }
            if i % 2 == 0 && c != '1' { res1 += 1; }
            if i % 2 != 0 && c != '1' { res0 += 1; }
            if i % 2 != 0 && c != '0' { res1 += 1; }
        }
        return cmp::min(res0, res1);
    }
}
