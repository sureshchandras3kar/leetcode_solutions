use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u32; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut count = [0u32; 26];
            for c in s.chars() {
                count[(c as u8 - b'a') as usize] += 1;
            }
            groups.entry(count).or_default().push(s);
        }
        groups.into_values().collect()
    }
}
