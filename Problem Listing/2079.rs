// https://leetcode.com/problems/watering-plants/
// 0 ms, 2.1 MB

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut res = 0;
        let mut pos:i32 = -1;
        let mut waterUnits = capacity;
        for x in 0..plants.len() {
            if waterUnits >= plants[x] {
                res += (pos-x as i32).abs();
                waterUnits -= plants[x];
            } else {
                res += (((pos+1)*2)+1).abs();
                waterUnits = capacity - plants[x];
            }
            pos = x as i32;
        }
        return res;
    }
}
