class Solution:
  def three_sum(nums):
    result = set()
    nums.sort()
    for i in range(len(nums)):
      p1 = i + 1
      p2 = len(nums) - 1
      while p1 < p2:
        s = nums[p1] + nums[p2] + nums[i]
        if s == 0:
          result.add((numd[p1], nums[p2], nums[i]))
          p1 += 1
        elif s > 0:
          p2 -= 1
        else:
          p1 += 1
    return [list(x) for x in result]