// https://leetcode.com/problems/maximum-units-on-a-truck/
// 8 ms, 2.2 MB

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut occupied = 0;
        let mut res = 0;
        let mut vec = box_types;
        vec.sort_by(|a, b| b[1].cmp(&a[1]));
        for x in vec {
            for y in 0..x[0] {
                if occupied < truck_size {
                    res += x[1];
                    occupied += 1;
                } else {
                    return res;
                }
            }
        }
        return res;
    }
}
