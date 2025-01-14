import unittest
from typing import *


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if len(prices) == 0:
            return 0

        max_profit: int = 0
        min_price = prices[0]

        for price in prices:
            min_price = min(min_price, price)
            max_profit = max(max_profit, price - min_price)

        return max_profit


class TestBestTimeToBuyAndSellStock(unittest.TestCase):
    def test_case_1(self):
        prices = [7, 1, 5, 3, 6, 4]
        result = 5
        self.assertEqual(result, Solution().maxProfit(prices))

    def test_case_2(self):
        prices = [7, 6, 4, 3, 1]
        result = 0
        self.assertEqual(result, Solution().maxProfit(prices))

    def test_case_3_empty_prices(self):
        prices = []
        result = 0
        self.assertEqual(result, Solution().maxProfit(prices))


if __name__ == "__main__":
    unittest.main()
