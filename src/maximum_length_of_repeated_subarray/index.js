/**
 * @param {number[]} A
 * @param {number[]} B
 * @return {number}
 */
var findLength = function (A, B) {
    let dp = [];
    for (let i = 0; i < A.length; i++) {
        let t = [];
        for (let j = 0; j < B.length; j++) {
            t.push(0);
        }
        dp.push(t);
    }
    let max_value = 0;
    for (let i = 0; i < A.length; i++) {
        for (let j = 0; j < B.length; j++) {
            if (A[i] === B[j]) {
                let t;
                if (i === 0 || j === 0) {
                    t = 0;
                } else {
                    t = dp[i - 1][j - 1]
                }
                dp[i][j] = t + 1;
                max_value = Math.max(max_value, dp[i][j]);
            } else {
                // dp[i][j] = Math.max(
                //     i === 0 ? 0 : dp[i - 1][j],
                //     j === 0 ? 0 : dp[i][j - 1]
                // );
            }
        }
    }
    return max_value;
};