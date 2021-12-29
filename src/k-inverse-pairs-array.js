/**
 * @param {number} n
 * @param {number} k
 * @return {number}
 */
// [1]

// [1, 2]
// [2, 1]

// [1, 2, 3]
// [1, 3, 2]  [2, 1, 3]   3, 1
// [3, 1, 2]  [2, 3, 1]
// [3, 2, 1]

// **3
// *3*
// 3**

// ***4 
// **4*
// *4**
// 4***

// [1, 2, 3, 4]                  1
// [1, 2, 4, 3]  [1, 3, 2, 4]  [2, 1, 3, 4]  dp[i][j] = dp[i-1][j] + (dp[i-2][i-1] * dp[i-3][i-1])
// [1, 4, 2, 3]  [1, 4, 3, 2]  [3, 1, 2, 4]  [2, 3, 1, 4]
// [3, 2, 1, 4]  [1, 4, 3, 2]  [4, 2, 1, 3]

var kInversePairs = function (n, k) {
    const MOD = 1000000007;
    let dp = [];
    for (let i = 0; i <= n; i++) {
        dp[i] = [];
        for (let j = 0; j <= k; j++) {
            dp[i][j] = 0;
        }
    }
    // n  k
    dp[0][0] = 1;

    for (let i = 1; i <= n; i++) {
        for (let j = 0; j <= k; j++) {
            for (let l = 0; l < i; l++) {
                if (j - l >= 0) {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j - l]) % MOD;
                }
            }
        }
    }
    return dp[n][k];
};

// var kInversePairs = function (n, k) {
//     if (n * (n - 1) / 2 < k)
//         return 0;

//     let dp = [1], mod = 1e9 + 7;

//     for (let i = 2; i <= n; i++) {
//         let temp = [1], max = i * (i - 1) / 2;

//         for (let j = 1; j <= max && j <= k; j++)
//             temp[j] = ((dp[j] || 0) + temp[j - 1] - (dp[j - i] || 0) + mod) % mod;

//         dp = temp;
//     }
//     return dp[k];
// };

console.log(
    // kInversePairs(1, 0),
    // kInversePairs(1, 1),
    // kInversePairs(2, 0),
    kInversePairs(1, 1),
    // kInversePairs(2, 2),
    // kInversePairs(3, 0)
);

