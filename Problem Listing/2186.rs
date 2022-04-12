// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram-ii/
// 139 ms, 2.7 MB (well, that sucks)

use std::collections::HashMap;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut res = 0;
        let mut counterS: HashMap<char, i32> = HashMap::new();
        let mut counterT: HashMap<char, i32> = HashMap::new();
        
        for letter in s.chars() {
            let count = counterS.entry(letter).or_insert(0);
            *count+=1;
        }
        
        for letter in t.chars() {
            let count = counterT.entry(letter).or_insert(0);
            *count+=1;
        }
        
        for (key, val) in counterS.iter() {
            let c = match counterT.get(key) {
                Some(x) => x,
                None => &0
            };
            res += (val-c).abs();
        }
        
        for (key, val) in counterT.iter() {
            let c = match counterS.get(key) {
                Some(x) => x,
                None => &0
            };
            if c == &0 {
                res += (val-c).abs();
            }
        }
        
        return res;
    }
}
