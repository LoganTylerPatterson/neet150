class Solution:
  def rotate(matrix):
    n = len(matrix)
    
    newMatrix = [[] for _ in range(len(matrix[0]))]
    
    for col in range(n):
      for row in range(n):
        newMatrix[col].append(matrix[row][col]])
    
    for row in matrix:
      row = row[::-1]
    
    return newMatrix
    