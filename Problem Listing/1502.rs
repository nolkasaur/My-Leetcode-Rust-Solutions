// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
// 1 ms, 2.1 MB

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let mut diff = arr[1] - arr[0];
        for x in 1..arr.len() {
            if arr[x] - arr[x-1] != diff {
                return false;
            }
        }
        return true;
    }
}
