// https://leetcode.com/problems/minimum-time-visiting-all-points/
// 2 ms, 2.1 MB

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for x in 0..points.len()-1 {
            let diffL = (points[x][0] - points[x+1][0]).abs();
            let diffR = (points[x][1] - points[x+1][1]).abs();
            if diffL > diffR {
                res += diffL;
            } else {
                res += diffR;
            }
        }
        return res;
    }
}
