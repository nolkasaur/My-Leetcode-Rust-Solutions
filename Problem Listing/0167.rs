// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// 313 ms, 2.3 MB

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for x in 0..numbers.len()-1 {
            for y in x+1..numbers.len() {
                if numbers[x]+numbers[y] == target {
                    return vec![x as i32 + 1, y as i32 + 1];
                } else if numbers[x]+numbers[y] > target {
                    break;
                }
            }
        }
        return vec![];
    }
}
