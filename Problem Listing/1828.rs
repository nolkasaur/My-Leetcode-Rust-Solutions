// https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/
// 25 ms, 2.2 MB

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::with_capacity(queries.len());
        for q in &queries {
            let mut nrOfPointsInside = 0;
            for p in &points {
                if (((p[1] - q[1]) * (p[1] - q[1]) + (p[0] - q[0]) * (p[0] - q[0])) as f32).sqrt() <= q[2] as f32{
                    nrOfPointsInside += 1;
                }
            }
            res.push(nrOfPointsInside);
        }
        return res;
    }
}
