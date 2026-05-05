class Solution:
    def isValid(self, s: str) -> bool:
        if not s or len(s) == 1: return False
        open = ['(', '[', '{']
        close = [')', ']', '}']
        pairs = ["()", "[]", '{}']
        stack = []
        chars = list(s)

        for char in chars:
            if char in open:
                stack.append(char)
            elif char in close:
                if stack:
                    last = stack.pop()
                    pair = last + char

                    if pair  not in pairs:
                        return False
                else:
                    return False
        if not stack:
            return True
        else:
            return False