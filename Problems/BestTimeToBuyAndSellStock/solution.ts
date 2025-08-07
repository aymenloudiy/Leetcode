function maxProfit(prices: number[]): number {
  let profit: number = 0;
  let buyPrice: number = prices[0];

  for (let i: number = 1; i < prices.length; i++) {
    if (buyPrice > prices[i]) {
      buyPrice = prices[i];
    }

    if (profit < prices[i] - buyPrice) {
      profit = prices[i] - buyPrice;
    }
  }

  return profit;
}
