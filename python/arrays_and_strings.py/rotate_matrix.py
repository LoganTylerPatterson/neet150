class Solution:
  def rotate(matrix):
    n = len(matrix)
    
    newMatrix = [[0 for _ in range(n)] for _ in range(n)]
    
    for row in range(n):
      for col in range(n):
        newMatrix[col][row] = matrix[row][col]
    
    for row in matrix:
      row = row[::-1]
    
    return newMatrix
    