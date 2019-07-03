/**
 * @param {number[]} nums
 * @return {number}
 */
var lengthOfLIS = function (nums) {
    let res = 0;
    if (nums.length === 0) {
        return 0;
    }
    if (nums.length === 1) {
        return 1;
    }
    let map = {};
    function run(nums) {
        let len = nums.length;
        if (map[len] != null) {
            return map[len]
        }
        let max = 1;
        for (let i = 0; i < len - 1; i++) {
            if (nums[i] < nums[len - 1]) {
                max = Math.max(max, run(nums.slice(0, i + 1)) + 1)
            }
        }
        res = Math.max(res, max)
        if (map[len] == null) {
            map[len] = max
        }
        console.log(nums, max)
        return max;
    }
    // run(nums);
    for (let i = 0; i < nums.length; i++) {
        run(nums.slice(0, i + 1))
    }
    return res;
};
// [4,10,4,3,8,9]