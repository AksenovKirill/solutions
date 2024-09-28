function maxProfit(prices: number[]): number {
    let temp = 0;
    let min = prices[0];

    for (const price of prices) {
        min = Math.min(min, price);
        temp = Math.max(temp, price - min);
    }

    return temp;
}
