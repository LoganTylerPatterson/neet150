impl Solution {
  pub fn encode(words: Vec<String>) -> String {
    let mut res: Vec<String> = Vec::new();
    for w in words {
      res.push(w.len().to_string());
      res.push("#".to_string());
      res.push(w);
    }
    return res.join("");
  }
  
  pub fn decode(s: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut i = 0;
    while i < s.len() {
      let mut j = i;
      while j < s.len() && s[j] == "#" {
        j += 1;
      }
      let length:i32 = s[i..j].parse().expect("Failed to parse length");
      i = j;
      res.push(s[i..j+1+length]);
    }
    res
  }
}