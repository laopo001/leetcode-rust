/**
 * @param {number} n
 * @return {number}
 */
// 缓存
var calculate = function (low, high, arr) {
    if (low >= high)
        return 0;
    if (arr[low][high] > 0) return arr[low][high];
    let minres = Infinity;
    for (let i = Math.floor((low + high) / 2); i <= high; i++) {
        let res = i + Math.max(calculate(i + 1, high, arr), calculate(low, i - 1, arr));
        minres = Math.min(res, minres);
    }
    return minres;
}

var getMoneyAmount = function (n) {
    let a = new Array(n).fill(new Array(n).fill(0));
    return calculate(1, n, a);
};
// 182 ms, faster than 69.44%
var getMoneyAmount = function (n) {

    let dp = new Array(n).fill(0).map(x => {
        return new Array(n).fill(Infinity)
    });
    for (let i = 0; i < n; i++) {
        dp[i][i] = 0;
    }
    for (let i = 1; i < n; i++) {
        for (let j = i - 1; j >= 0; j--) {
            for (let k = j; k <= i; k++) {
                let left = k == j ? 0 : dp[j][k - 1];
                let right = k == i ? 0 : dp[k + 1][i]
                dp[j][i] = Math.min((k + 1) + Math.max(left, right), dp[j][i]);
            }

        }
    }
    return dp[0][n - 1];
};

let res = getMoneyAmount(3);
console.log(res);