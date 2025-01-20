import heapq

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freqMap = {}
        for num in nums:
            freqMap[num] = freqMap.get(num, 0) + 1
        
        result = []
        for key, value in freqMap.items():
            heappush(result, (value, key))
            if len(result) > k:
                heappop(result)
        
        return [x[1] for x in result]