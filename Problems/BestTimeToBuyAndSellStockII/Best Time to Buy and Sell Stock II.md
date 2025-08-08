---
created: 2025-08-08
modified: 
completed: true
leetcode-index: 122
link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii
difficulty: Medium
tags:
  - leetcode/array
  - leetcode/dynamic-programming
  - leetcode/greedy
  - leetcode/problem
---
# Best Time to Buy and Sell Stock II

## Problem Statement
You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `i^th` day.

On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

Find and return *the maximum profit you can achieve*.

 

>[!Example]+ Example 1
>**Input**: `prices = [7,1,5,3,6,4]`
>**Output**: `7`
>**Explanation**:
>Buy on day 2 (price = 
> 1) and sell on day 3 (price = 
> 5) , profit = 5-1 = 4. Then buy on day 4 (price = 
> 3) and sell on day 5 (price = 
> 6) , profit = 6-3 = 3. Total profit is 4 + 3 = 7. 

>[!Example]+ Example 2
>**Input**: `prices = [1,2,3,4,5]`
>**Output**: `4`
>**Explanation**:
>Buy on day 1 (price = 
> 1) and sell on day 5 (price = 
> 5) , profit = 5-1 = 4. Total profit is 4. 

>[!Example]+ Example 3
>**Input**: `prices = [7,6,4,3,1]`
>**Output**: `0`
>**Explanation**:
>There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0. 

>[!warning]+ Constraints
>- `1 <= prices.length <= 3 * 10^4`
>
>- `0 <= prices[i] <= 10^4`
## Hints
No hints available.
## Approach

- Selling and buying depends on profits made between each consecutive days, so whenever the profit chain breaks we dont need to sell and can hold until we can make profit again.
- If 2 consecutive days make profit, increase the profit else ignore
## Solution

```go
# Solution
func maxProfit(prices []int) int {
    profit := 0
    for i:= 1; i < len(prices);i++ {
        if prices[i]-prices[i-1] > 0 {
            profit += prices[i] - prices[i-1]
        }
    }
    return profit
}
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
- This was surprisingly easy for a medium.