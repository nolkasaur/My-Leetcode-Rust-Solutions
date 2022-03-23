// https://leetcode.com/problems/sort-even-and-odd-indices-independently/
// 4 ms, 2 MB

impl Solution {
    pub fn sort_even_odd(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut oddArr = Vec::new();
        let mut evenArr = Vec::new();
        let mut res = Vec::with_capacity(len);
        let mut y = 0;
        let mut z = 0;
        for x in 0..len {
            if x % 2 == 0 {
                evenArr.push(nums[x]);
            } else {
                oddArr.push(nums[x]);
            }
        }
        oddArr.sort();
        oddArr.reverse();
        evenArr.sort();
        for x in 0..len {
            if x % 2 == 0 {
                res.push(evenArr[y]);
                y +=1 ;
            } else {
                res.push(oddArr[z]);
                z +=1 ;
            }
        }
        return res;
    }
}
