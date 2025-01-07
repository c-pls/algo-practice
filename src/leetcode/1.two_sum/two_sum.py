from typing import List
import unittest


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        res: List[int] = []

        m = {}

        for idx, e in enumerate(nums):
            if target - e in m:
                return [idx, m.get(target - e)]

            m[e] = idx

        return res


class TestTwoSum(unittest.TestCase):
    def test_basic_case(self):
        self.assertEqual(sorted(Solution().twoSum([2, 7, 11, 15], 9)), [0, 1])

    def test_multiple_solutions(self):
        self.assertEqual(sorted(Solution().twoSum([3, 2, 4], 6)), [1, 2])

    def test_no_solution(self):
        self.assertEqual(Solution().twoSum([1, 2, 3], 7), [])

    def test_negative_numbers(self):
        self.assertEqual(sorted(Solution().twoSum([-1, -2, -3, -4], -6)), [1, 3])


if __name__ == "__main__":
    unittest.main()
