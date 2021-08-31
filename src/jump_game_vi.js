/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */

// Time Limit Exceeded
var maxResult = function (nums, k) {
    let dp = [];
    let res = -Infinity;
    for (let i = 1; i < nums.length; i++) {
        dp[i] = -Infinity;
    }
    dp[0] = nums[0];

    // dp[1] = dp[0] + nums[1];
    for (let i = 1; i < nums.length; i++) {

        for (j = 1; j <= k; j++) {
            if (i - j >= 0) {
                dp[i] = Math.max(dp[i], dp[i - j] + nums[i]);
            }
        }
    }
    return dp[nums.length - 1];
};

maxResult([1, -1, -2, 4, -7, 3], 2)

maxResult([10, -5, -2, 4, 0, 3], 3)

maxResult([1, -5, -20, 4, -1, 3, -6, -3], 2)

maxResult([100, -100, -300, -300, -300, -100, 100], 4)
