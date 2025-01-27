use std::collections::HashMap;

impl Solution {
  pub fn vaild_sudoku(grid: Vec<Vec<i32>>) -> bool {
    let mut rows = vec![HashMap::new()];
    let mut cols = vec![HashMap::new()];
    let mut nines = vec![HashMap::new()];
    
    for row in 0..grid.len() {
      for col in 0..row.len() {
        let num = grid[row][col];
        if num == '.'{
          continue
        }
        let num = grid[row][col] as usize - '1' as usize;
        let ninesIndex = (row / 3) * 3 + col / 3;
        if rows[row].contains_key(&num) || cols[col].contains_key(&num) || nines[ninesIndex].contains_key(&num){
          false;
        }
        rows[row].insert(num, true);
        cols[col].insert(num, true);
        nines[ninesIndex].insert(num, true);
      }
    }
    true
  }
}