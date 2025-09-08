function maxProfit(prices: number[]): number {
  let profit: number = 0;
  let start = Number.MAX_SAFE_INTEGER;

  for (let i: number = 0; i < prices.length; i++) {
    if (prices[i] < start) {
      start = prices[i];
    }
    profit = prices[i] - start > profit ? prices[i] - start : profit;
  }

  return profit;
}
