class Solution:
  
  def one_away(self, s1, s2):
    if abs(len(s1)) - abs(len(s2)) > 1:
      return False
    
    larger = s1
    smaller = s2
    if len(s1) < len(s2):
      larger = s2
      smaller = s1
    
    # replace
    count = 0
    if len(s1) == len(s2):
      for i in range(len(smaller)):
        if smaller[i] != larger[i]:
          count += 1
        if count > 1:
          break
      
      if count < 2:
        return True
      
    # remove or insert
    j = 0
    i = 0
    while i < range(len(smaller)) and abs(i-j) < 2:
      if larger[j] != smaller[i]:
        j += 1
      i+=1
      j+=1
        
    return True if abs(i-j) < 2 else False