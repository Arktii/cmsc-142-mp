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

export const topDown = (capacity, items) => {
  const n = items.length;
  const memo = Array.from({ length: n }, () => Array(capacity + 1).fill(-1));
  return { value: topDownRec(capacity, items, n - 1, memo) };
};

const topDownRec = (capacity, items, index, memo) => {
  if (index < 0) {
    return 0;
  }

  // if already computed --> memoized
  if (memo[index][capacity] !== -1) {
    return memo[index][capacity];
  }

  if (items[index].weight > capacity) {
    memo[index][capacity] = topDownRec(capacity, items, index - 1, memo);
  } else {
    memo[index][capacity] = Math.max(
      items[index].value +
        topDownRec(capacity - items[index].weight, items, index - 1, memo),
      topDownRec(capacity, items, index - 1, memo)
    );
  }

  return memo[index][capacity];
};
