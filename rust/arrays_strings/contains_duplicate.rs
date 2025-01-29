use std::collections::HashSet;

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>){
    let mut seen = HashSet::new();
    
    for n in nums {
      if seen.contains(n) {
        return true;
      }
      seen.insert(n);
    }
    return false;
  }
}