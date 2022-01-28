// https://leetcode.com/problems/duplicate-zeros/
// 4 ms, 2.2 MB

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut x = 0;
        while x < arr.len() {
            if arr[x] == 0 {
                arr.insert(x+1, 0);
                x += 1;
                arr.remove(arr.len()-1);
            }
            x += 1;
        }
    }
}
