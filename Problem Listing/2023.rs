// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
// 55 ms, 2.2 MB

impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut res = 0;
        let len = nums.len();
        for x in 0..len {
            for y in 0..x {
                if format!("{}{}", nums[x], nums[y]) == target { res += 1; }
            }
            for z in x+1..len {
                if format!("{}{}", nums[x], nums[z]) == target { res += 1; }
            }
        }
        return res;
    }
}
