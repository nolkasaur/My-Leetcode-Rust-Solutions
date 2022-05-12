// https://leetcode.com/problems/arithmetic-subarrays/
// 31 ms, 2.1 MB

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut res: Vec<bool> = Vec::with_capacity(l.len());
        for x in 0..l.len() {
            let mut sortedSubArray = &mut nums.clone()[l[x] as usize..r[x] as usize+1];
            sortedSubArray.sort();
            let dist = sortedSubArray[1]-sortedSubArray[0];
            for y in 0..sortedSubArray.len()-1 {
                if sortedSubArray[y+1]-sortedSubArray[y] != dist {
                    res.push(false);
                    break;
                }
                if y == sortedSubArray.len()-2 {
                    res.push(true);
                }
            }
        }
        return res;
    }
}
