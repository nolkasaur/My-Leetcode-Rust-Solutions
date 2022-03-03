// https://leetcode.com/problems/count-good-triplets/
// 4 ms, 2 MB

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut res = 0;
        for x in 0..arr.len()-2 {
            for y in x+1..arr.len()-1 {
                for z in y+1..arr.len() {
                    if (arr[x] - arr[y]).abs() <= a &&
                    (arr[y] - arr[z]).abs() <= b &&
                    (arr[x] - arr[z]).abs() <= c {
                        res += 1;
                    }
                }
            }
        }
        return res;
    }
}
