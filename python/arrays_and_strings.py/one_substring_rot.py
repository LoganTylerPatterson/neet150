class Solution:
  def oneSubstringRot(s, t):
    ns = s + s
    if t in s:
      return True
    return False