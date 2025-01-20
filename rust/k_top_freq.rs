use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
         let mut freq_map = HashMap::new();

        for n in nums {
            *freq_map.entry(n).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();

        for (num, count) in freq_map {
            heap.push((-count, num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.into_iter().map(|(_, num)| num).collect()
    }
}