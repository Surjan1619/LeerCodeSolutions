class Solution:
    def romanToInt(self, s: str) -> int:
        values = {
            'I': 1,
            'V': 5,
            'X': 10,
            'L': 50,
            'C': 100,
            'D': 500,
            'M': 1000
        }
        txt = list(s)
        st = len(txt) - 1
        res = 0
        while st >= 0:
            if st == len(txt) - 1:
                res += values[txt[st]]
                st -= 1
                continue
            if values[txt[st]] < values[txt[st + 1]]:
                print(txt[st], txt[st + 1], "aa")
                print(values[txt[st]], "bb")
                res -= values[txt[st]]
                st -= 1
                continue
            if values[txt[st]] >= values[txt[st + 1]]:
                res += values[txt[st]]
                st -= 1
                continue
        return res