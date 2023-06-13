/**
 * Definition for isBadVersion()
 * 
 * @param {integer} version number
 * @return {boolean} whether the version is bad
 * isBadVersion = function(version) {
 *     ...
 * };
 */

// 69 ms, faster than 77.68%
/**
 * @param {function} isBadVersion()
 * @return {function}
 */
var solution = function (isBadVersion) {
    /**
     * @param {integer} n Total versions
     * @return {integer} The first bad version
     */
    function guess(n) {
        if (isBadVersion(n)) {
            if (!isBadVersion(n - 1)) {
                return 0;
            } else {
                return -1;
            }
        } else {
            return 1;
        }
    }
    return function (n) {
        if (guess(0) === 0) {
            return 0;
        }
        if (guess(n) === 0) {
            return n;
        }
        let left = 0;
        let right = n;
        let middle = Math.floor((left + right) / 2);
        while (1) {
            if (guess(middle) === 1) {
                left = middle;
                middle = Math.floor((left + right) / 2);
            } else if (guess(middle) === -1) {
                right = middle;
                middle = Math.floor((left + right) / 2);
            } else {
                return middle;
            }
        }
    };
};