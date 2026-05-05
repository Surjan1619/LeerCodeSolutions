class Solution:
    def reverse(self, x: int) -> int:
        is_negative = False
        if x < 0:
            is_negative = True
            x = str(x)[1:][::-1]
            if int(x) > 2**31 - 1 or int(x) < -2**31:
                return 0
            else:
                return int(x) * -1
        elif x == 0:
            return 0
        else:
            x = int(str(x)[::-1])
            if x > 2**31 - 1 or x < -2**31:
                return 0
            else:
                return x