class Solution:
  def compression(s):
    res = []
    curr = 0
    for i in range(len(s)):
      if s[i] != s[curr]:
        count = i - curr
        res.append(str(curr))
        res.append(str(s[i]))
        curr = i
    if len(res) < len(s):
      return "".join()
    return s
    
if __name__ == '__main__':
  sol = Solution()
  print(sol.compression("abaababbbcccccc"))