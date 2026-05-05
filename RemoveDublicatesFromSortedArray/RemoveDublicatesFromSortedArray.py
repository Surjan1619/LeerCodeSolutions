from typing import List
class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        uniq = list(set(nums))
        uniq.sort()
        nums[:len(uniq)] = list(uniq)
        return len(uniq)