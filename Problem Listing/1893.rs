// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/
// 3 ms, 2.1 MB

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        'outer: for x in left..right+1 {
            for y in &ranges {
                if y[0] <= x && y[1] >= x {
                    continue 'outer;
                }
            }
            return false;
        }
        return true;
    }
}
