// https://leetcode.com/problems/find-all-duplicates-in-an-array/
// 16 ms, 3.4 MB

use std::collections::HashMap;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return [].to_vec();
        }
        let mut res: Vec<i32> = Vec::new();
        let mut numMap: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let count = numMap.entry(num).or_insert(0);
            *count+=1;
            if *count == 2 {
                res.push(num);
            }
        }
        return res;
    }
}
