/**
 * @param {string} s
 * @param {string} t
 * @param {number} k
 * @return {boolean}
 */

// 252 ms, faster than 10.00%
var canConvertString = function (s, t, k) {
    if (s.length != t.length) return false;
    let res = [];
    debugger
    for (let i = 0; i < s.length; i++) {
        const a = s[i];
        const b = t[i];
        let z = b.charCodeAt(0) - a.charCodeAt(0);
        if (z < 0) {
            z = z + 26;
        }
        if (z != 0) {
            res.push(z);
        }
    }
    let map = {};
    res.sort();
    let x = 0;
    let c = 0;
    for (let i = 0; i < res.length; i++) {
        const element = res[i];
        if (element != x) {
            x = element;
            c = 1;
        } else {
            c++;
        }
        if (element + 26 * (c - 1) > k) {
            return false;
        }
        if (map[element + 26 * (c - 1)]) {
            return false;
        } else {
            map[element + 26 * (c - 1)] = true;
        }
    }
    return true;
};

// 104 ms, faster than 44.44%
var canConvertString = function (s, t, k) {
    if (s.length != t.length) return false;
    let res = [];
    for (let i = 0; i < s.length; i++) {
        const a = s[i];
        const b = t[i];
        let z = b.charCodeAt(0) - a.charCodeAt(0);
        if (z < 0) {
            z = z + 26;
        }
        if (z != 0) {
            res.push(z);
        }
    }
    let map = {};
    for (let i = 0; i < res.length; i++) {
        const element = res[i];
        let c = map[element] || 1;
        if (element + 26 * (c - 1) > k) {
            return false;
        }
        if (map[element + 26 * (c - 1)]) {
            return false;
        } else {
            c++;
            map[element] = c;
        }
    }
    return true;
};

canConvertString("atmtxzjkz", "tvbtjhvjd", 35)

canConvertString("abc", "bcd", 10)

