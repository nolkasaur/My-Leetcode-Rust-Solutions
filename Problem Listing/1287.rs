// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/
// 4 ms, 2.2 MB

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut res = arr[0];
        let mut count = 1;
        for num in 1..arr.len() {
            if arr[num] != arr[num-1] {
                count = 1;
                res = arr[num];
            } else {
                count += 1;
                if count > arr.len()/4 {
                    return arr[num]
                }
            }
        }
        return res;
    }
}
