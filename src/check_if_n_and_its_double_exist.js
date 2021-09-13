/**
 * @param {number[]} arr
 * @return {boolean}
 */
// 80 ms, faster than 52.98%
var checkIfExist = function (arr) {
    let map = {};
    for (let i = 0; i < arr.length; i++) {
        const element = arr[i];
        if (element == 0) {
            if (map[element]) {
                return true;
            }
            map[element] = true;
            continue;
        }
        if (map[element * 2] || map[element / 2]) {
            return true
        }
        map[element] = true;
    }
    return false;
};