// https://leetcode.com/problems/check-if-number-has-equal-digit-count-and-digit-value/
// 4 ms, 2.2 MB

use std::collections::HashMap;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for x in 0..num.len() {
            let count = counts.entry(num.chars().nth(x).unwrap().to_digit(10).unwrap()).or_insert(0);
            *count+=1;
        }
        for x in 0..num.len() {
            let count = match counts.get(&(x as u32)) {
                Some(val) => *val,
                None => 0
            };
            if count != num.chars().nth(x).unwrap().to_digit(10).unwrap(){
                return false;
            }
        }
        return true;
    }
}
