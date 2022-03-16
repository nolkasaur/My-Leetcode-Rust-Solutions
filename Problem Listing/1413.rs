// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/
// 0 ms, 2 MB

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut startValue = 1;
        'outer: loop {
            let mut sum = startValue;
            for x in &nums {
                sum += x;
                if sum < 1 {
                    startValue += 1;
                    continue 'outer;
                }
            }
            return startValue;
        }
    }
}
