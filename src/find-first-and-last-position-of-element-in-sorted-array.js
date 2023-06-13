/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
// 112 ms, faster than 20.86% 
var searchRange = function (nums, target) {
    let start = -1
    let end = -1;
    for (let i = 0; i < nums.length; i++) {
        const element = nums[i];
        if (element == target && start == -1) {
            start = i;
            end = i;
        } else if (element == target && start != -1) {
            end = i;
        }
    }
    return [start, end]
};


// 68 ms, faster than 94.99% 二分法
var searchRange = function (nums, target) {
    let n = nums.length - 1;
    let res = -1;
    function guess(n) {
        if (nums[n] < target) {
            return 1;
        } else if (nums[n] > target) {
            return -1;
        } else {
            return 0;
        }
    }

    if (guess(n) == 0) {
        res = n;
    } else if (guess(n) == 1) {
        return [-1, -1];
    } else {
        let left = 0;
        let right = n;
        let middle = Math.floor((left + right) / 2);
        while (left < right) {
            let v = guess(middle);
            if (v == 1) {
                if (left === middle) {
                    break;
                }
                left = middle;
                middle = Math.floor((left + right) / 2);
            } else if (v == -1) {
                if (right === middle) {
                    break;
                }
                right = middle;
                middle = Math.floor((left + right) / 2);
            } else {
                res = middle;
                break;
            }
        }
    }
    if (res == -1) {
        return [-1, -1];
    }
    let start = res;
    let end = res;
    while (nums[start - 1] == target) {
        start -= 1;
    }
    while (nums[end + 1] == target) {
        end += 1;
    }
    return [start, end];
};

searchRange([1, 2, 3], 1);
