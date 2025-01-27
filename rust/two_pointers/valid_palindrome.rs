impl Solution {
  pub fn valid_panidrome(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut p1 = 0;
    let mut p2 = s.len() - 1;
    
    while p1 <= p2 {
      if s[p1] != s[p2] {
        false
      }
      p1 += 1;
      p2 -= 1;
    }

    true
  }
}