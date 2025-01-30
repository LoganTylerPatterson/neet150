impl Solution {
  pub fn two_sum(nums:Vec<i32>, target: i32){
    let mut p1 = 0;
    let mut p2 = nums.len() - 1;
    while p1 < p2 {
      let sum = nums[p1] + nums[p2];
      if sum > target {
        p2 -= 1;
      } else if sum < target {
        p2 += 1;
      } else {
        return vec![(p1 + 1) as i32, (p2 + 1) as i32];
      }
    }
    return vec![-1,-1];
  }
}