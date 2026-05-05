class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        if nums:
            for findex, first in enumerate(nums):
                for sindex, second in enumerate(nums):
                    if findex != sindex:
                        if first + second == target:
                            print(first, second)
                            return [findex, sindex]
        else:
            return []
        return []
