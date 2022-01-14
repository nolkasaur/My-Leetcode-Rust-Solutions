// https://leetcode.com/problems/di-string-match/
// 7 ms, 2.2 MB

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let len = s.len();
        let mut min: i32 = 0;
        let mut max: i32 = len as i32;
        let mut res: Vec<i32> = Vec::with_capacity(len+1);
        for x in s.chars() {
            if x == 'I' {
                res.push(min);
                min+=1;
            } else {
                res.push(max);
                max-=1;
            }
        }
        if s.chars().nth(len-1).unwrap() == 'I' {
            res.push(min);
        } else {
            res.push(max);
        }
        return res;
    }
}
