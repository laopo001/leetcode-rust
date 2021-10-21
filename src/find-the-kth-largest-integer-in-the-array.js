/**
 * @param {string[]} nums
 * @param {number} k
 * @return {string}
 */

// 108 ms, faster than 99.15% 
function compareNumStr(a, b) {
    if (a.length > b.length) {
        return 1
    } else if (a.length < b.length) {
        return -1
    }
    if (a > b) {
        return 1
    } else if (a < b) {
        return -1
    }
    return 0
}

var kthLargestNumber = function (nums, k) {
    nums = nums.sort((a, b) => {
        return compareNumStr(b, a)
    });
    return nums[k - 1];
};