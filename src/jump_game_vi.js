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

// 维护一个单调队列，这里的队列是双端队列
class Dequeue {
    constructor() {
        this.list = [];
    }

    push(val) {
        // console.log(val)
        // 保证数据从队头到队尾递减
        while (this.list[this.list.length - 1] < val) {
            this.list.pop(); // 其余数据于本题无用，丢弃
        }
        this.list.push(val); // 插入到适当位置
    }

    // 队头出队
    shift(val) {
        if (this.list[0] === val) { // 按题意出队，不满足当前是最大值的情况下不出队
            // 这里的js实现shift()理论上复杂度应该是O(k), 就不去真实实现一个O(1)出队的队列了，意思到位即可
            this.list.shift();
        }
    }

    // peek查看队列首端的最大值
    max() {
        return this.list[0];
    }
}



var maxResult = function (nums, k) {
    let dp = [];
    let res = -Infinity;
    for (let i = 1; i < nums.length; i++) {
        dp[i] = -Infinity;
    }
    dp[0] = nums[0];
    let q = new Dequeue();
    q.push(nums[0]);

    for (let i = 1; i < nums.length; i++) {

        // q.push(nums[i]);
        // dp[i] = dp[i - k] + q.max();

        if (Math.max(i - k, 0)) { // start为max(i-k, 0)
            q.shift(dp[i - k - 1]); // 移除start的前一个
        }
        dp[i] = q.max() + nums[i]
        q.push(dp[i])

    }
    return dp[nums.length - 1];
};

maxResult([1, -1, -2, 4, -7, 3], 2)

maxResult([10, -5, -2, 4, 0, 3], 3)

maxResult([1, -5, -20, 4, -1, 3, -6, -3], 2)

maxResult([100, -100, -300, -300, -300, -100, 100], 4)
