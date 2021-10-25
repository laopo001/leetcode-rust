/**
 * @param {string} s
 * @return {number}
 */

// 122 ms, faster than 71.43% 
var longestAwesome = function (s) {
    let map = {};
    let res = 0;
    let q = 0;
    map[res] = -1;
    for (let i = 0; i < s.length; i++) {
        const num = +s[i] - 0;
        q = q ^ (1 << num)
        for (let j = 0; j < 10; j++) {
            let right = q ^ (1 << j);
            if (map[right] != null) {
                res = Math.max(res, i - map[right])
            }
        }
        if (map[q] != null) {
            res = Math.max(res, i - map[q])
        } else {
            map[q] = i;
        }

    }

    return res;
};


var res = longestAwesome('11')
console.log(res);
