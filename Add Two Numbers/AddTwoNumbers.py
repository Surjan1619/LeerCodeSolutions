
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        def get_v(ls):
            res = []
            def rec(ls):
                res.append(ls.val)
                if ls.next:
                    rec(ls.next)
            rec(ls)
            return res
        l1 = get_v(l1)[::-1]
        l2 = get_v(l2)[::-1]
        l1 = int("".join(map(str, l1)))
        l2 = int("".join(map(str, l2)))
        sum = l1 + l2
        index = 0
        sum = [int(i) for i in list(str(sum))]
        #lista mejy tver
        while sum:
            if index > len(sum) - 1:
                break
            else:
                val = sum[index]
                if index > 0:
                    next = sum[index - 1]
                else:
                    next = None
                sum[index] = ListNode(val=val, next=next)
                index = index + 1
        return sum[-1]



a = ListNode(1)
b = ListNode(2)
c = ListNode(3)

d = ListNode(4)
e = ListNode(5)
f = ListNode(6)
a.next = b
b.next = c
d.next = e
e.next = f
solution = Solution()
print(solution.addTwoNumbers(a, d))