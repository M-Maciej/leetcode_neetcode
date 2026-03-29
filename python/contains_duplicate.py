from typing import List

from sortedcontainers import SortedSet


class Solution:
    def hash_simple(self, nums: List[int]) -> bool:
        seen = set()
        for a in nums:
            if a in seen:
                return True
            seen.add(a)
        return False

    def hash_iter(self, nums: List[int]) -> bool:
        return len(set(nums)) != len(nums)

    def sort_simple(self, nums: List[int]) -> bool:
        nums.sort()
        return any(nums[i] == nums[i + 1] for i in range(len(nums) - 1))

    def dict_simple(self, nums: List[int]) -> bool:
        seen = {}
        for n in nums:
            if n in seen:
                return True
            seen[n] = True
        return False

    from sortedcontainers import SortedSet

    def sorted_set_iter(self, nums: List[int]) -> bool:
        seen = SortedSet()
        for n in nums:
            if n in seen:
                return True
            seen.add(n)
        return False


if __name__ == "__main__":
    sol = Solution()

    functions_to_test = {
        sol.hash_simple,
        sol.hash_iter,
        sol.sort_simple,
        sol.dict_simple,
        sol.sorted_set_iter,
    }

    for func in functions_to_test:
        num = [1, 2, 3, 1]
        num2 = [1, 2, 3, 4]
        assert func(list(num)), f"{func.__name__} failed T1"
        assert not func(list(num2)), f"{func.__name__} failed T2"

        print(f"{func.__name__} passed!")
