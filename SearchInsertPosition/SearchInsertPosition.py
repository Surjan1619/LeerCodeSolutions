from typing import List
#numss = [2, 5, 8, 12, 16, 23, 38, 42, 50, 55, 62, 70, 84, 91, 100, 101, 102]
class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        result = None
        l = 0
        r = len(nums) - 1
        if target <= nums[l]:
            return 0
        elif target > nums[r]:
            return len(nums)
        while l <= r:
            print(l, r, "___", nums[l], nums[r])
            middle = (l + r) // 2
            if target == nums[middle]:
                return middle

            elif target > nums[middle]:
                l = middle
            elif target < nums[middle]:
                r = middle
            if r - l == 1:
                result = l + 1
                return result