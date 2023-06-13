/**
 * @param {string} s
 * @return {boolean}
 */
var areNumbersAscending = function (s) {
    let nums = s.split(' ').map(x => +x).filter(n => !isNaN(n));
    for (let i = 0; i < nums.length - 1; i++) {
        if (nums[i] >= nums[i + 1]) {
            return false;
        }
    }
    return true;
};