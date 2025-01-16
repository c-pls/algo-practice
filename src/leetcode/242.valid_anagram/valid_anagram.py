import unittest
from typing import *


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        return Counter(s) == Counter(t)

    def isAnagramSort(self, s: str, t: str) -> bool:
        return sorted(s) == sorted(t)


class TestValidAnagram(unittest.TestCase):
    def test_sample_1(self):
        s = "anagram"
        t = "nagaram"

        self.assertEqual(Solution().isAnagram(s, t), True)

    def test_sample_2(self):
        s = "rat"
        t = "car"

        self.assertEqual(Solution().isAnagram(s, t), False)

    def test_sample_3(self):
        s = "a"
        t = "a"

        self.assertEqual(Solution().isAnagram(s, t), True)

    def test_sample_4(self):
        s = ""
        t = "nagaram"

        self.assertEqual(Solution().isAnagram(s, t), False)

    def test_sample_5(self):
        s = "anagram"
        t = "nagaram"

        self.assertEqual(Solution().isAnagramSort(s, t), True)

    def test_sample_6(self):
        s = "rat"
        t = "car"

        self.assertEqual(Solution().isAnagramSort(s, t), False)

    def test_sample_7(self):
        s = "a"
        t = "a"

        self.assertEqual(Solution().isAnagramSort(s, t), True)

    def test_sample_8(self):
        s = ""
        t = "nagaram"

        self.assertEqual(Solution().isAnagramSort(s, t), False)


if __name__ == "__main__":
    unittest.main()
