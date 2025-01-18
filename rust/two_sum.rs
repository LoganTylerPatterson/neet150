use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cm = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate(){
          let complement = target - num;
          if cm.contains_key(&complement){
            return vec![*cm.get(&complement).unwrap() as i32, i as i32];
          }
          cm.insert(num, i);
        }
        
        return vec![-1,-1];
    }
}