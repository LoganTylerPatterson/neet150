use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut wordMap = HashMap::new();

        for word in strs {
            let mut collection: Vec<char> = word.chars().collect();
            collection.sort();
            let sortedWord: String = collection.iter().collect();
            
            wordMap.entry(sortedWord).or_insert(vec![]).push(word);
        }

        wordMap.into_values().collect();
}
