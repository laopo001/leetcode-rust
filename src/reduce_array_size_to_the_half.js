/**
 * @param {number[]} arr
 * @return {number}
 */
// 1768 ms, faster than 5.55%
var minSetSize = function (arr) {
    let len = arr.length;
    let map = {};
    for (let i = 0; i < arr.length; i++) {
        const element = arr[i];
        if (map[element] != null) {
            map[element]++;
        } else {
            map[element] = 1
        }
    }
    let array = [];
    for (const key in map) {
        array.push(map[key]);
    }
    array.sort((a, b) => b - a);
    let l = len;
    let res = 0;
    while (l > len / 2) {
        let first = array.shift();
        l -= first;
        res++;
    }
    return res;
};