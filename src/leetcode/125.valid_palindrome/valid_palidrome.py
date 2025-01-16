import unittest
from typing import *


class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = "".join(c for c in s.lower() if c.isalnum())
        left = 0
        right = len(s) - 1
        middle = right / 2

        while left < middle:
            if s[left] != s[right]:
                return False
            left += 1
            right -= 1

        return True

    def isPalindromeRevert(self, s: str) -> bool:
        s = "".join(c for c in s.lower() if c.isalnum())
        return s == s[::-1]


class TestValidPalidrome(unittest.TestCase):
    def test_sample_1(self):
        s = "A man, a plan, a canal: Panama"
        self.assertEqual(Solution().isPalindrome(s), True)

    def test_sample_2(self):
        s = "race a car"
        self.assertEqual(Solution().isPalindrome(s), False)

    def test_sample_3(self):
        s = "    "
        self.assertEqual(Solution().isPalindrome(s), True)

    def test_sample_4(self):
        s = "0P"
        self.assertEqual(Solution().isPalindrome(s), False)


if __name__ == "__main__":
    unittest.main()
