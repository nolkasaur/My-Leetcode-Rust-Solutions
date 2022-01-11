// https://leetcode.com/problems/jewels-and-stones/

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut count = 0;
        for x in 0..jewels.len() {
            count += stones.matches(jewels.chars().nth(x).unwrap()).count();
        }
        return count as i32;
    }
}
