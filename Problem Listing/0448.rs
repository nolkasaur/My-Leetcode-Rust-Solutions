// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
// 20 ms, 2.8 MB

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut sorted = nums;
        sorted.sort();
        let mut currNumToFind = 1;
        for n in 0..sorted.len() {
            if sorted[n] == currNumToFind {
                currNumToFind += 1;
            } else if sorted[n] > currNumToFind {
                for x in currNumToFind..sorted[n] {
                    res.push(x);
                    currNumToFind += 1;
                }
                currNumToFind += 1;
            }
        }
        for y in currNumToFind..(sorted.len()+1) as i32 {
            res.push(y)
        }
        return res;
    }
}
