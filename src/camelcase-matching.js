/**
 * @param {string[]} queries
 * @param {string} pattern
 * @return {boolean[]}
 */

// 80 ms, faster than 44.44%
var camelMatch = function (queries, pattern) {
    return queries.map(x => compare(x, pattern))
};

function compare(a, b) {
    let j = 0;
    let i = 0;
    let start = 0;
    while (i < a.length) {
        if (a[i].charCodeAt(0) >= '65' && a[i].charCodeAt(0) <= '90') {
            if (b[j] == null) {
                return false;
            }
            if (b[j].charCodeAt(0) >= '65' && b[j].charCodeAt(0) <= '90') { // a大写 b大写
                if (a[i] == b[j]) {
                    i++;
                    j++;
                } else {
                    return false;
                }
            } else { //  a大写 b不是大写
                return false;
            }
        } else {
            if (b[j] == null || b[j].charCodeAt(0) >= '65' && b[j].charCodeAt(0) <= '90') { // a不是大写 b大写
                i++;
            } else { // a不是大写 b不是大写
                if (a[i] == b[j]) {
                    i++;
                    j++;
                } else {
                    i++;
                }
            }
        }
    }
    if (j < b.length) {
        return false;
    }
    return true;
}
console.log(
    compare('FaoBar', 'FoBa')
);
