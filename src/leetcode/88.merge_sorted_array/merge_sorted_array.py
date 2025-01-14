import unittest
from typing import *


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        if m == 0 and n == 0:
            return
        i = m - 1
        j = n - 1
        k = m + n - 1

        while i >= 0 and j >= 0:
            if nums1[i] > nums2[j]:
                nums1[k] = nums1[i]
                i -= 1
            else:
                nums1[k] = nums2[j]
                j -= 1
            k -= 1

        if j < 0:
            return

        while j >= 0:
            nums1[k] = nums2[j]
            k -= 1
            j -= 1


class TestMergeSortedArray(unittest.TestCase):
    def test_sample_1(self):
        nums1 = [1, 2, 3, 0, 0, 0]
        m = 3
        nums2 = [2, 5, 6]
        n = 3

        Solution().merge(nums1, m, nums2, n)

        self.assertEqual(nums1, [1, 2, 2, 3, 5, 6])

    def test_sample_2(self):
        nums1 = [1]
        m = 1
        nums2 = []
        n = 0

        Solution().merge(nums1, m, nums2, n)

        self.assertEqual(nums1, [1])

    def test_sample_3(self):
        nums1 = [1, 5, 8, 9, 0, 0, 0, 0]
        m = 4
        nums2 = [0, 4, 6, 7]
        n = 4

        Solution().merge(nums1, m, nums2, n)

        self.assertEqual(nums1, [0, 1, 4, 5, 6, 7, 8, 9])

    def test_sameple_4(self):
        nums1 = [0]
        m = 0
        nums2 = [1]
        n = 1

        Solution().merge(nums1, m, nums2, n)

        self.assertEqual(nums1, [1])


if __name__ == "__main__":
    unittest.main()
