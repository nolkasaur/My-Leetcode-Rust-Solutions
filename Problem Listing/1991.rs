// https://leetcode.com/problems/find-the-middle-index-in-array/
// 4 ms, 2 MB

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        } else if len == 2 {
            if nums[1] == 0 {
                return 0;
            } else if nums[0] == 0 {
                return 1;
            }
        } else {
            let mut countX = 0;
            for x in 1..len {
                countX += nums[x];
            }
            if countX == 0 {
                return 0;
            }
            for z in 1..len-1 {
                let mut countZL = 0;
                let mut countZR = 0;
                for i in 0..z {
                    countZL += nums[i];
                }
                for j in z+1..len {
                    countZR += nums[j];
                }
                if countZL == countZR {
                    return z as i32;
                }
            }
            let mut countY = 0;
            for y in 0..len-1 {
                countY += nums[y];
            }
            if countY == 0 {
                return (len-1) as i32;
            }
        }
        return -1;
    }
}
