class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        text = list(s)
        if len(text) == 1:
            return 1
        res = []
        index = 0
        temp = []
        while True:
            if index == len(text):
                break
            while temp:  # himnakany vor
                if index == len(text):
                    res.append(temp)
                    break
                for i in temp:  # frum
                    if text[index] == text[i]:  # nayu
                        index = i + 1  # ete chk
                        res.append(temp)
                        temp = []
                        break
                temp.append(index)
                index += 1
            else:
                temp.append(index)
                index += 1
        if res:
            res = max(res, key=len)
            return len(res)
        else:
            return 0
# r = []
# dct = {1:"a", 2:"b", 3 :"c", 4 :"d", 5 :"e"}
# for i in dct:
#     print(dct[i])
#     if dct[i] == "d":
#         r.append(dct[i])
#         r.append(i)
# print(r)




# g h u i w f
# 0 1 2 3 4 5
#
# f w e
# 6 7 8