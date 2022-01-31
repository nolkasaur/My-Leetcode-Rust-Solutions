// https://leetcode.com/problems/second-largest-digit-in-a-string/
// 5 ms, 2.3 MB

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut vec: Vec<i32> = Vec::new();
        for char in s.chars() {
            if char.is_digit(10) { vec.push(char.to_digit(10).unwrap() as i32) }
        }
        vec.sort();
        vec.dedup();
        if vec.len() == 0 || vec.len() == 1 { return -1 }
        return vec[vec.len()-2];
    }
}
