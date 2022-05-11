// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
// 64 ms, 9.8 MB

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = 0;
        for x in 0..points.len()-1 {
            if points[x+1][0] - points[x][0] > res {
                res = points[x+1][0] - points[x][0];
            }
        }
        return res;
    }
}
