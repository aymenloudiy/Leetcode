---
created: 2025-08-07
modified: 
completed: true
leetcode-index: 121
link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock
difficulty: Easy
tags:
  - leetcode/array
  - leetcode/dynamic-programming
  - leetcode/problem
---
# Best Time to Buy and Sell Stock

## Problem Statement
You are given an array `prices` where `prices[i]` is the price of a given stock on the `i^th` day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return *the maximum profit you can achieve from this transaction*. If you cannot achieve any profit, return `0`.

 

>[!Example]+ Example 1
>**Input**: `prices = [7,1,5,3,6,4]`
>**Output**: `5`
>**Explanation**:
>Buy on day 2 (price = 
> 1) and sell on day 5 (price = 
> 6) , profit = 6-1 = 5. Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell. 

>[!Example]+ Example 2
>**Input**: `prices = [7,6,4,3,1]`
>**Output**: `0`
>**Explanation**:
>In this case, no transactions are done and the max profit = 0. 

>[!warning]+ Constraints
>- `1 <= prices.length <= 10^5`
>
>- `0 <= prices[i] <= 10^4`
## Hints
No hints available.
## Approach

-  Loop through the array and check for higher price.
- At the same time check if profit is higher or lower.
## Solution

```ts
# Solution
function maxProfit(prices: number[]): number {
    let profit:number = 0;
    let buyPrice:number = prices[0];
    
    for (let i:number = 1; i < prices.length; i++) {
        if (buyPrice > prices[i]) {
            buyPrice = prices[i];
        }

        if (profit < prices[i] - buyPrice) {
            profit = prices[i] - buyPrice
        }
    }

    return profit;
};
```

## Complexity Analysis

- Time complexity: $$O(n)$$
- Space complexity: $$O(1)$$

## Reflections
- Could not solve it with double pointers.