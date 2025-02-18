class Solution:
  def biggestContainer(heights):
    p1, p2 = 0, len(heights) - 1
    result = 0
    while p1 < p2:
      area = (p2 - p1) * min(heights[p1], heights[p2])
      result = max(result, area)
      if heights[p1] <= heights[p2]:
        p1 += 1
      elif heights[p2] < heights[p1]:
        p2 -= 1
    return result