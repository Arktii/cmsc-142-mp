// https://www.geeksforgeeks.org/0-1-knapsack-problem-dp-10/#tabulation-or-bottomup-approach-on-x-w-time-and-space

export const bottomUp = (capacity, items) => {
  const n = items.length;
  const dp = Array.from({ length: n + 1 }, () => Array(capacity + 1).fill(0));

  for (let i = 0; i <= n; i++) {
    for (let w = 0; w <= capacity; w++) {
      if (i === 0 || w === 0) {
        dp[i][w] = 0;
      } else if (items[i - 1].weight <= w) {
        dp[i][w] = Math.max(
          items[i - 1].value + dp[i - 1][w - items[i - 1].weight],
          dp[i - 1][w]
        );
      } else {
        dp[i][w] = dp[i - 1][w];
      }
    }
  }

  return { value: dp[n][capacity] };
};
