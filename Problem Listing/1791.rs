// https://leetcode.com/problems/find-center-of-star-graph/
// 37 ms, 8.9 MB

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let aa = edges[0][0];
        let ab = edges[0][1];
        let ba = edges[1][0];
        let bb = edges[1][1];
        if aa == ba || aa == bb { aa }
        else { ab }
    }
}
