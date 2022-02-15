// https://leetcode.com/problems/maximum-score-after-splitting-a-string/
// 545 ms, 2 MB - well... that could be more efficient

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut res = 0;
        for x in 0..s.len()-1 {
            let mut sumAux = 0;
            for l in 0..x+1 {
                if s.chars().nth(l).unwrap() == '0' { sumAux += 1; }
            }
            for r in x+1..s.len() {
                if s.chars().nth(r).unwrap() == '1' { sumAux += 1; }
            }
            if sumAux > res { res = sumAux; }
        }
        return res;
    }
}
