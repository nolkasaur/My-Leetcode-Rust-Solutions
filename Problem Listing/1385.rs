// https://leetcode.com/problems/find-the-distance-value-between-two-arrays/
// 0 ms, 2.3 MB

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut res = 0;
        'outer: for x in &arr1 {
            for y in &arr2 {
                if (x-y).abs() <= d {
                    continue 'outer;
                }
            }
            res += 1;
        }
        return res;
    }
}
