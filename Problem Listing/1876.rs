// https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/submissions/
// 3 ms, 2.2 MB

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }
        let mut res = 0;
        for x in 0..s.len()-2 {
            if (s.chars().nth(x).unwrap() != s.chars().nth(x+1).unwrap()) && (s.chars().nth(x).unwrap() != s.chars().nth(x+2).unwrap()) && (s.chars().nth(x+1).unwrap() != s.chars().nth(x+2).unwrap()) {
                res += 1;
            }
        }
        return res;
    }
}
