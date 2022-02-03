// https://leetcode.com/problems/delete-columns-to-make-sorted/
// 608 ms, 2.3 MB (well that's mighty inefficient...)

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut res = 0;
        'outer: for x in 0..strs[0].len() {
            let mut charToCompare = 'a' as u32;
            for str in &strs {
                let c = str.chars().nth(x).unwrap() as u32;
                if c < charToCompare {
                    res += 1;
                    continue 'outer;
                } else {
                    charToCompare = c;
                }
            }
        }
        return res;
    }
}
