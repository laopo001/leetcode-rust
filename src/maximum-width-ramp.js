/**
 * @param {number[]} nums
 * @return {number}
 */
var maxWidthRamp = function (nums) {
    let left = 0;
    let right = nums.length - 1;
    return run(nums, left, right);
};

function run(nums, left, right) {
    let res = 0;
    if (left < right) {
        if (nums[left] <= nums[right]) {
            res = right - left;
        } else {
            if (left + 1 == right) {
                return res;
            }
            return Math.max(
                run(nums, left + 1, right),
                run(nums, left, right - 1)
            )
        }
    }
    return res;
}
// 84 ms, faster than 97.87%
var maxWidthRamp = function (nums) {
    let res = 0;
    let arr = [0];
    for (let i = 1; i < nums.length; i++) {
        if (nums[arr[arr.length - 1]] > nums[i]) {
            arr.push(i);
        }
    }
    for (let i = nums.length - 1; i > 0; i--) {
        // for (let j = 0; j < arr.length; j++) {
        //     if (nums[i] >= nums[arr[j]]) {
        //         res = Math.max(res, i - arr[j]);
        //         break;
        //     }
        // }
        while (!arr.length == 0 && nums[arr[arr.length - 1]] <= nums[i]) {
            let pos = arr.pop();
            res = Math.max(res, i - pos);
        }

    }
    return res;
};


var res = maxWidthRamp([9, 8, 1, 0, 1, 9, 4, 0, 4, 1])
console.log(res);