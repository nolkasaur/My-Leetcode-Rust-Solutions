// https://leetcode.com/problems/destination-city/
// 3 ms, 2.1 MB

use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut mappings: HashMap<String, String> = HashMap::new();
        for x in &paths {
            mappings.insert(x[0].to_string(), x[1].to_string());
        }
        return findDestination(&mappings, &paths[0][0]);
    }
}

pub fn findDestination(map: &HashMap<String, String>, s: &String) -> String {
    println!("{}", s);
    match &map.get(s) {
        Some(city) => findDestination(&map, &city.to_string()),
        _ => return s.to_string(),
    }
}
