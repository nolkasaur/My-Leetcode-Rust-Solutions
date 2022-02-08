// https://leetcode.com/problems/subdomain-visit-count/submissions/
// 7 ms, 2.4 MB

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut counter: HashMap<String, u32> = HashMap::new();
        for domain in cpdomains {
            let mut splt1: Vec<&str> = domain.split(' ').collect();
            let mut splt2: Vec<&str> = splt1[1].split('.').collect();
            let n = &splt2.len();
            if *n == 2 {
                let count1 = counter.entry(format!("{}.{}", splt2[0], splt2[1])).or_insert(0);
                *count1 += splt1[0].parse::<u32>().unwrap();
                let count2 = counter.entry(format!("{}", splt2[1])).or_insert(0);
                *count2 += splt1[0].parse::<u32>().unwrap();
            } else {
                let count1 = counter.entry(format!("{}.{}.{}", splt2[0], splt2[1], splt2[2])).or_insert(0);
                *count1 += splt1[0].parse::<u32>().unwrap();
                let count2 = counter.entry(format!("{}.{}", splt2[1], splt2[2])).or_insert(0);
                *count2 += splt1[0].parse::<u32>().unwrap();
                let count3 = counter.entry(format!("{}", splt2[2])).or_insert(0);
                *count3 += splt1[0].parse::<u32>().unwrap();
            }
        }
        let mut res: Vec<String> = Vec::with_capacity(counter.keys().len());
        for (k, v) in counter {
            res.push(format!("{} {}", v, k));
        }
        return res;
    }
}
