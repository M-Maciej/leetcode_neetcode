class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        counts = {}
        for a, b in zip(s, t):
            counts[a] = counts.get(a, 0) + 1
            counts[b] = counts.get(b, 0) - 1
        return all(val == 0 for val in counts.values())

    def isAnagramCheat(self, s: str, t: str) -> bool:
        from collections import Counter

        return Counter(t) == Counter(s)


if __name__ == "__main__":
    sol = Solution()
    word = "xx1122aaaddd"
    word2 = "xx2121aadadd"
    word3 = "xx1122aaad"

    functions = {sol.isAnagram, sol.isAnagramCheat}

    for func in functions:
        assert func(str(word), str(word2)), f"{func.__name__} failed T1"
        assert not func(str(word), str(word3)), f"{func.__name__} failed T2"
        print(f"{func.__name__} passed!")
