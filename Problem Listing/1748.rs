// https://leetcode.com/problems/sum-of-unique-elements/
// 0 ms, 2 MB

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 { return nums[0]; }
        else if len == 2 {
            if nums[0] == nums[1] { return 0; }
            else { return nums[0]+nums[1]; }
        }
        let mut res = 0;
        let mut count = 0;
        let mut nrRemoved = 0;
        let mut sorted: Vec<i32> = nums;
        let mut dup = false;
        sorted.sort();
        sorted.push(-1);
        sorted.push(-1);
        while count < len - nrRemoved {
            if sorted[count+1] == sorted[count] {
                sorted.remove(count+1);
                nrRemoved+=1;
                dup = true;
            } else {
                if !dup { res+=sorted[count]; }
                else { dup = false; }
                count+=1;
            }
        }
        return res;
    }
}
