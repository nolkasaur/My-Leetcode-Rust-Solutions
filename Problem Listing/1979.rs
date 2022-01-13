// https://leetcode.com/problems/find-greatest-common-divisor-of-array/
// 4 ms, 2 MB

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();
        for x in (1..*min+1).rev() {
            if max%x==0 && min%x==0 { return x; }
        }
        return -1;
    }
}
