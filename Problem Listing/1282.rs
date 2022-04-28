// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
// 4 ms, 2.2 MB

use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut mappings: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut res: Vec<Vec<i32>> = Vec::new();
        for x in 0..group_sizes.len() {
            let y = mappings.entry(group_sizes[x]).or_insert(Vec::new());
            y.push(x as i32);
        }
        for  (k, v) in mappings.iter() {
            let mut x = 0;
            while x < v.len() {
                let mut aux: Vec<i32> = Vec::new();
                for y in x..x+*k as usize {
                    aux.push(v[y as usize]);
                }
                res.push(aux);
                x += *k as usize;
            }
        }
        return res;
    }
}
