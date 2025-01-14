import unittest
from typing import *


class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        needle_length = len(needle)
        haystack_length = len(haystack)

        if needle_length == 0:
            return 0

        for i in range(haystack_length - needle_length + 1):
            if haystack[i] == needle[0]:
                h_ptr, n_ptr = i, 0
                while n_ptr < needle_length and haystack[h_ptr] == needle[n_ptr]:
                    h_ptr += 1
                    n_ptr += 1

                    if n_ptr == needle_length:
                        return i

        return -1


class TestFindTheIndexOfTheFirstOccurrenceInAString(unittest.TestCase):
    def test_sample_1(self):
        haystack = "sadbutsad"
        needle = "sad"

        res = Solution().strStr(haystack, needle)

        self.assertEqual(res, 0)

    def test_sample_2(self):
        haystack = "leetcode"
        needle = "leeto"

        res = Solution().strStr(haystack, needle)

        self.assertEqual(res, -1)

    def test_sample_3(self):
        haystack = "adbutsad"
        needle = "sad"

        res = Solution().strStr(haystack, needle)

        self.assertEqual(res, 5)

    def test_sample_4(self):
        haystack = "a"
        needle = "a"

        res = Solution().strStr(haystack, needle)

        self.assertEqual(res, 0)


if __name__ == "__main__":
    unittest.main()
