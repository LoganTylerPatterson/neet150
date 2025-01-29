use std::collections::HashSet;

impl Solution {
  pub fn longest_sequence(nums: Vec<i32>) -> i32 {
      let mut nSet = HashSet::new();
      for n in nums {
        nSet.insert(n);
      }
      
      let mut max = 0;
      for n in nums {
        if nSet.contains(&(n-1)) {
          continue;
        }
        let mut x = n;
        while nSet.contains(&x) {
          x += 1;
        }
        max = math.max(x - n);
      }
      max
  }
}